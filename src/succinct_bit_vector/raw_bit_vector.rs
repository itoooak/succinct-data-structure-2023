pub struct BitVector {
    pub data: Vec<u64>,
    pub len: usize,
}

impl BitVector {
    pub fn new(len: usize) -> Self {
        BitVector {
            data: vec![0; (len + 64 - 1) / 64],
            len,
        }
    }

    pub fn from_vec(data: Vec<u64>) -> Self {
        let len = data.len() * 64;
        BitVector { data, len }
    }

    pub fn from_boolvec(data_bool: Vec<bool>) -> Self {
        let len = data_bool.len();
        let mut data = vec![];

        for i in 0..(len / 64) {
            let mut v = 0;
            for j in 0..64 {
                v |= (data_bool[i * 64 + j] as u64) << j;
            }
            data.push(v);
        }

        if len % 64 != 0 {
            let mut v = 0;
            for j in 0..(len % 64) {
                v |= (data_bool[len / 64 * 64 + j] as u64) << j;
            }
            data.push(v);
        }

        BitVector { data, len }
    }

    pub fn get(self: &Self, i: usize) -> bool {
        self.data[i / 64] & (1 << (i % 64)) != 0
    }

    pub fn get_64(self: &Self, i: usize) -> u64 {
        self.data[i]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bit_vector_create() {
        let bv = super::BitVector::from_vec(vec![0b101010101]);
        for i in 0..9 {
            assert_eq!(bv.get(i), i % 2 == 0);
        }

        for i in 9..64 {
            assert_eq!(bv.get(i), false);
        }
    }

    #[test]
    fn test_bit_vector_from_boolvec() {
        let data = vec![true, false, true, false, true, true];
        let bv = super::BitVector::from_boolvec(data.clone());
        for i in 0..bv.len {
            assert_eq!(bv.get(i), data[i]);
        }
    }
}
