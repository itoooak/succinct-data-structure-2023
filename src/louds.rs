use std::vec;

use crate::succinct_bit_vector::SucBV;

type OrderedTree = Vec<Vec<usize>>;

pub struct Louds {
    _tree: OrderedTree,
    bit_vector: SucBV,
}

impl Louds {
    pub fn new(tree: &OrderedTree) -> Self {
        let mut bv_raw = vec![true, false];

        for i in 0..tree.len() {
            let mut tmp = vec![true; tree[i].len() + 1];
            tmp[tree[i].len()] = false;
            bv_raw.append(&mut tmp);
        }

        assert_eq!(bv_raw.len(), tree.len() * 2 + 1);

        let bit_vector = SucBV::from_boolvec(bv_raw);

        Louds { _tree: tree.to_vec(), bit_vector }
    }

    pub fn bfs_rank(self: &Self, x: usize) -> usize {
        self.bit_vector.rank(x, true)
    }

    pub fn bfs_select(self: &Self, i: usize) -> usize {
        self.bit_vector.select(i, true).unwrap()
    }

    pub fn parent_rank(self: &Self, x: usize) -> usize {
        self.bit_vector.rank(x - 1, false)
    }

    pub fn first_child_select(self: &Self, i: usize) -> usize {
        self.bit_vector.select(i, false).unwrap() + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let tree: OrderedTree = vec![
            vec![1, 2], vec![3, 4, 5], vec![6, 7], vec![], vec![], vec![8], vec![9], vec![], vec![], vec![],
        ];

        let louds = Louds::new(&tree);

        let raw_louds: Vec<bool> = "101101110110001010000".chars().into_iter().map(|c| c == '1').collect();
        let len = raw_louds.len();

        for i in 0..len {
            assert_eq!(louds.bit_vector.access(i), raw_louds[i], " at {} th loop", i);
        }
    }

    #[test]
    fn test_rank_select() {
        let tree: OrderedTree = vec![
            vec![1, 2], vec![3, 4, 5], vec![6, 7], vec![], vec![], vec![8], vec![9], vec![], vec![], vec![],
        ];

        let louds = Louds::new(&tree);

        for i in 0..tree.len() {
            let x = louds.bfs_select(i);
            assert_eq!(i, louds.bfs_rank(x));

            let i2 = louds.bfs_rank(x);
            assert_eq!(x, louds.bfs_select(i2));

            assert_eq!(i, i2);
        }
    }

    // TODO: parent_rank, first_child_selectのテスト
}
