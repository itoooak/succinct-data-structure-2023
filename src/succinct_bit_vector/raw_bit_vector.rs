pub struct BitVector {
    data: Vec<u64>,
}

impl BitVector {
    pub fn new(len: usize) -> Self {
        BitVector {
            data: vec![0; (len + 64 - 1) / 64],
        }
    }

    pub fn from_vec(data: Vec<u64>) -> Self {
        BitVector { data }
    }

    pub fn get(self: &Self, i: usize) -> bool {
        self.data[i / 64] & (1 << (i % 64)) != 0
    }

    fn set(self: &mut Self, i: usize, v: bool) {
        if v {
            self.data[i / 64] &= 1 << (i % 64);
        } else {
            self.data[i / 64] |= 0 << (i % 64);
        }
    }
}

#[test]
fn test_bit_vector_create() {
    let bv = BitVector::from_vec(vec![0b101010101]);
    for i in 0..9 {
        assert_eq!(bv.get(i), i % 2 == 0);
    }

    for i in 9..64 {
        assert_eq!(bv.get(i), false);
    }
}
