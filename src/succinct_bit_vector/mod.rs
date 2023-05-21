use std::arch::x86_64::_popcnt64;

mod raw_bit_vector;
use self::raw_bit_vector::BitVector;

mod rank_index;
use self::rank_index::RankIndex;
pub const RANK_INDEX_LARGE_SIZE: usize = 1 << 16;
pub const RANK_INDEX_SMALL_SIZE: usize = 1 << 8;

pub struct SucBV {
    bit_vector: BitVector,
    rank_index: RankIndex,
}

impl SucBV {
    pub fn new(len: usize) -> Self {
        let bit_vector = BitVector::new(len);
        let rank_index = RankIndex::create(&bit_vector);
        SucBV {
            bit_vector,
            rank_index,
        }
    }

    pub fn from_vec(data: Vec<u64>) -> Self {
        let bit_vector = BitVector::from_vec(data);
        let rank_index = RankIndex::create(&bit_vector);
        SucBV {
            bit_vector,
            rank_index,
        }
    }

    pub fn from_boolvec(data_bool: Vec<bool>) -> Self {
        let bit_vector = BitVector::from_boolvec(data_bool);
        let rank_index = RankIndex::create(&bit_vector);
        SucBV {
            bit_vector,
            rank_index,
        }
    }

    pub fn access(self: &Self, i: usize) -> bool {
        self.bit_vector.get(i)
    }

    // [0, i)
    pub fn rank(self: &Self, i: usize, j: bool) -> usize {
        let mut sum1: usize = self.rank_index.large[i / RANK_INDEX_LARGE_SIZE]
            + self.rank_index.small[i / RANK_INDEX_SMALL_SIZE];

        let l = (i - i % RANK_INDEX_SMALL_SIZE) / 64;
        let r = i / 64;

        for k in l..r {
            sum1 += unsafe { _popcnt64(self.bit_vector.get_64(k) as i64) as usize }
        }

        for k in 0..(i % 64) {
            sum1 += if self.access(r * 64 + k) { 1 } else { 0 };
        }

        if j {
            sum1
        } else {
            i - sum1
        }
    }

    // i: 0-indexed
    pub fn select(self: &Self, i: usize, j: bool) -> Option<usize> {
        // iが1-indexedの方がやりやすいと思ったので変換
        let i = i + 1;

        if i <= 0 {
            None
        } else {
            let i_large = self.rank_index.large.partition_point(|&x| x < i);

            let remaining = i - self.rank_index.large[i_large - 1];
            let i_small_start = (i_large - 1) * RANK_INDEX_LARGE_SIZE / RANK_INDEX_SMALL_SIZE;
            let i_small_end = i_large * RANK_INDEX_LARGE_SIZE / RANK_INDEX_SMALL_SIZE;
            let i_small_end = if i_small_end > self.rank_index.small.len() { self.rank_index.small.len() } else { i_small_end };

            let i_small = self.rank_index.small[i_small_start..i_small_end].partition_point(|&x| x < remaining);
            let mut remaining = remaining - self.rank_index.small[i_small_start + i_small - 1];

            let i_bit_start = (i_small_start + i_small - 1) * RANK_INDEX_SMALL_SIZE;
            for k in i_bit_start..self.bit_vector.len {
                if self.access(k) {
                    remaining -= 1;
                }

                if remaining == 0 {
                    return Some(k);
                }
            }

            None
        }
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;
    const LENGTH: usize = (1 << 20) + 1000;

    #[test]
    fn test_access() {
        let mut rng = rand::thread_rng();
        let mut raw = vec![false; LENGTH];
        for i in 0..LENGTH {
            raw[i] = rng.gen();
        }

        let sucbv = super::SucBV::from_boolvec(raw.clone());
        for i in 0..LENGTH {
            assert_eq!(raw[i], sucbv.access(i), " at {} th loop", i);
        }
    }

    #[test]
    fn test_rank_0() {
        let mut rng = rand::thread_rng();
        let mut raw = vec![false; LENGTH];
        let mut sum0 = vec![0; LENGTH + 1];
        for i in 0..LENGTH {
            raw[i] = rng.gen();
            sum0[i + 1] = sum0[i] + !raw[i] as usize;
        }

        let sucbv = super::SucBV::from_boolvec(raw);

        for i in 0..=LENGTH {
            assert_eq!(sum0[i], sucbv.rank(i, false), " at {} th loop", i);
        }
    }

    #[test]
    fn test_rank_1() {
        let mut rng = rand::thread_rng();
        let mut raw = vec![false; LENGTH];
        let mut sum1 = vec![0; LENGTH + 1];
        for i in 0..LENGTH {
            raw[i] = rng.gen();
            sum1[i + 1] = sum1[i] + raw[i] as usize;
        }

        let sucbv = super::SucBV::from_boolvec(raw);

        for i in 0..=LENGTH {
            assert_eq!(sum1[i], sucbv.rank(i, true), " at {} th loop", i);
        }
    }

    #[test]
    fn test_select1() {
        let mut rng = rand::thread_rng();
        let mut raw = vec![false; LENGTH];
        let mut indices = Vec::new();
        let mut popcnt = 0;
        for i in 0..LENGTH  {
            raw[i] = rng.gen();
            if raw[i] {
                indices.push(i);
                popcnt += 1;
            }
        }
        let sucbv = super::SucBV::from_boolvec(raw);

        for i in 0..popcnt {
            assert_eq!(Some(indices[i]), sucbv.select(i, true), " at {} th loop", i);
        }

        for i in popcnt..LENGTH {
            assert_eq!(None, sucbv.select(i, true));
        }
    }
}
