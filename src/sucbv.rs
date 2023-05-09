use std::arch::x86_64::{_popcnt64, _pdep_u64, _tzcnt_u64};

pub struct SucBV {
    bit_vector: Box<[u32]>,
}

impl SucBV {
    pub fn new(vector: Box<[u32]>) -> Self {
        SucBV { bit_vector: vector }
    }

    pub fn access(self: &Self, i: usize) -> u32 {
        (self.bit_vector[i / 32] & (1 << (i % 32)) != 0) as u32
    }

    pub fn rank(self: &Self, i: u64, j: bool) {
        todo!();
    }

    pub fn select(self: &Self, i: u64, j: bool) {
        todo!();
    }
}
