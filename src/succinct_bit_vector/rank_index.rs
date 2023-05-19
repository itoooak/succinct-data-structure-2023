use std::arch::x86_64::_popcnt64;

use super::raw_bit_vector::BitVector;
use super::{RANK_INDEX_LARGE_SIZE, RANK_INDEX_SMALL_SIZE};

pub struct RankIndex {
    pub large: Vec<usize>,
    pub small: Vec<usize>,
}

impl RankIndex {
    pub fn create(bv: &BitVector) -> Self {
        let mut large = vec![];
        let mut small = vec![];

        let mut large_cnt = 0;
        let mut small_cnt = 0;
        for i in 0..(bv.len / 64) {
            if i % (RANK_INDEX_LARGE_SIZE / 64) == 0 {
                large.push(large_cnt);
                small_cnt = 0;
            }
            if i % (RANK_INDEX_SMALL_SIZE / 64) == 0 {
                small.push(small_cnt);
            }

            let cnt = unsafe { _popcnt64(bv.get_64(i) as i64) as usize };
            large_cnt += cnt;
            small_cnt += cnt;
        }

        if (bv.len / 64) % (RANK_INDEX_LARGE_SIZE / 64) == 0 {
            large.push(large_cnt);
            small_cnt = 0;
        }
        if (bv.len / 64) % (RANK_INDEX_SMALL_SIZE / 64) == 0 {
            small.push(small_cnt);
        }

        RankIndex { large, small }
    }
}
