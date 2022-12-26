// https://leetcode.com/problems/rank-transform-of-a-matrix/
use std::collections::BTreeMap;

// TODO: Add comments
pub struct Solution;

type Pos = (usize, usize);

pub struct DisjointSet {
    row_size: usize,
    ds: Vec<usize>,
}

impl DisjointSet {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let total_size = row_size + col_size;
        let mut ds = Vec::with_capacity(total_size);
        for root_idx in 0..total_size {
            ds.push(root_idx);
        }
        Self { row_size, ds }
    }

    pub fn connect(&mut self, row_idx: usize, col_idx: usize) {
        let row_root = self.get_root(row_idx);
        let col_root = self.get_root(self.row_size + col_idx);

        self.ds[col_root] = row_root;
    }

    pub fn get_root(&mut self, idx: usize) -> usize {
        let mut root = self.ds[idx];
        while root != self.ds[root] {
            root = self.ds[root];
        }
        return root;
    }
}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![Vec::new(); matrix.len()];
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut val_pos_treemap = BTreeMap::new();
        for row_idx in 0..rows {
            for col_idx in 0..cols {
                let val = matrix[row_idx][col_idx];
                val_pos_treemap
                    .entry(val)
                    .or_insert_with(|| Vec::new())
                    .push((row_idx, col_idx));
            }
        }

        let mut rank_matrix = vec![vec![0; cols]; rows];

        let mut curr_row_ranks = vec![0; rows];
        let mut curr_col_ranks = vec![0; cols];
        for (val, poses) in val_pos_treemap {
            let mut ds = DisjointSet::new(rows, cols);
            for (row_idx, col_idx) in &poses {
                ds.connect(*row_idx, *col_idx);
            }

            let mut max_root_ranks = vec![0; rows];
            for (row_idx, col_idx) in &poses {
                let root_idx = ds.get_root(*row_idx);
                max_root_ranks[root_idx] = max_root_ranks[root_idx]
                    .max(curr_row_ranks[*row_idx])
                    .max(curr_col_ranks[*col_idx]);
            }

            for (row_idx, col_idx) in poses {
                let root_idx = ds.get_root(row_idx);
                let new_rank = max_root_ranks[root_idx] + 1;
                rank_matrix[row_idx][col_idx] = new_rank;
                curr_row_ranks[row_idx] = new_rank;
                curr_col_ranks[col_idx] = new_rank;
            }
        }

        return rank_matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // assert_eq!(Solution::function(), 0);
        todo!();
    }
}
