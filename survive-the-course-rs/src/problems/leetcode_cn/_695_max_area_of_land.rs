// https://leetcode-cn.com/problems/max-area-of-island/

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                max_area = std::cmp::max(max_area, Self::spread(&mut grid, row, col));
            }
        }

        max_area
    }

    fn spread(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        let mut cells_to_process = VecDeque::from([(row, col)]);
        let mut area = 0;
        while let Some((row, col)) = cells_to_process.pop_front() {
            if grid[row][col] == -1 {
                // cells with value of -1 is visited
                continue;
            }
            if grid[row][col] == 0 {
                grid[row][col] = -1;
                continue;
            }

            grid[row][col] = -1;
            area += 1;

            if row > 0 {
                cells_to_process.push_back((row - 1, col));
            }
            if row < grid.len() - 1 {
                cells_to_process.push_back((row + 1, col));
            }
            if col > 0 {
                cells_to_process.push_back((row, col - 1));
            }
            if col < grid[0].len() - 1 {
                cells_to_process.push_back((row, col + 1));
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );
    }
}
