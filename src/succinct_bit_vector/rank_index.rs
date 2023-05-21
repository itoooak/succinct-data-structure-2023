use std::arch::x86_64::_popcnt64;

use super::raw_bit_vector::BitVector;
use super::{RANK_INDEX_LARGE_SIZE, RANK_INDEX_SMALL_SIZE};

pub struct RankIndex {
    pub large1: Vec<usize>,
    pub small1: Vec<usize>,
    pub large0: Vec<usize>,
    pub small0: Vec<usize>,
}

impl RankIndex {
    pub fn create(bv: &BitVector) -> Self {
        let mut large1 = vec![];
        let mut small1 = vec![];
        let mut large0 = vec![];
        let mut small0 = vec![];

        let mut large_cnt1 = 0;
        let mut small_cnt1 = 0;
        for i in 0..(bv.len / 64) {
            if i % (RANK_INDEX_LARGE_SIZE / 64) == 0 {
                large1.push(large_cnt1);
                large0.push(i * 64 - large_cnt1);
                small_cnt1 = 0;
            }
            if i % (RANK_INDEX_SMALL_SIZE / 64) == 0 {
                small1.push(small_cnt1);
                small0.push(
                    i * 64 - i * 64 / RANK_INDEX_LARGE_SIZE * RANK_INDEX_LARGE_SIZE - small_cnt1,
                );
            }

            let cnt = unsafe { _popcnt64(bv.get_64(i) as i64) as usize };
            large_cnt1 += cnt;
            small_cnt1 += cnt;
        }

        if (bv.len / 64) % (RANK_INDEX_LARGE_SIZE / 64) == 0 {
            large1.push(large_cnt1);
            large0.push(bv.len - large_cnt1);
            small_cnt1 = 0;
        }
        if (bv.len / 64) % (RANK_INDEX_SMALL_SIZE / 64) == 0 {
            small1.push(small_cnt1);
            small0
                .push(bv.len - bv.len / RANK_INDEX_LARGE_SIZE * RANK_INDEX_LARGE_SIZE - small_cnt1);
        }

        RankIndex {
            large1,
            small1,
            large0,
            small0,
        }
    }
}
