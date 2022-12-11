// https://leetcode.com/problems/swim-in-rising-water/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        // (elevation, row_idx, col_idx)
        let mut pq = BinaryHeap::new();
        // Push the top left cell into the pq
        pq.push(Reverse((grid[0][0], 0, 0)));
        let mut curr_elevation = grid[0][0];
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        while !pq.is_empty() {
            let Reverse((elevation, row_idx, col_idx)) = pq.pop().unwrap();
            if row_idx == rows - 1 && col_idx == cols - 1 {
                return curr_elevation.max(grid[rows - 1][cols - 1]);
            }
            if visited[row_idx][col_idx] {
                continue;
            }
            visited[row_idx][col_idx] = true;
            if elevation > curr_elevation {
                curr_elevation = grid[row_idx][col_idx];
            }

            if row_idx < rows - 1 {
                pq.push(Reverse((grid[row_idx + 1][col_idx], row_idx + 1, col_idx)));
            }

            if col_idx < cols - 1 {
                pq.push(Reverse((grid[row_idx][col_idx + 1], row_idx, col_idx + 1)));
            }

            if row_idx > 0 {
                pq.push(Reverse((grid[row_idx - 1][col_idx], row_idx - 1, col_idx)));
            }

            if col_idx > 0 {
                pq.push(Reverse((grid[row_idx][col_idx - 1], row_idx, col_idx - 1)));
            }
        }

        panic!("Impossible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::function(), 0);
    }
}
