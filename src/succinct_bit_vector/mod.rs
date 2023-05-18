use std::arch::x86_64::{_popcnt64, _pdep_u64, _tzcnt_u64};

pub mod raw_bit_vector;
use self::raw_bit_vector::BitVector;

pub struct SucBV {
    bit_vector: raw_bit_vector::BitVector,
}

impl SucBV {
    pub fn new(len: usize) -> Self {
        SucBV { bit_vector: BitVector::new(len) }
    }

    pub fn from_vec(data: Vec<u64>) -> Self {
        SucBV { bit_vector: BitVector::from_vec(data) }
    }

    pub fn access(self: &Self, i: usize) -> bool {
        self.bit_vector.get(i)
    }

    pub fn rank(self: &Self, i: u64, j: bool) {
        todo!();
    }

    pub fn select(self: &Self, i: u64, j: bool) {
        todo!();
    }
}
