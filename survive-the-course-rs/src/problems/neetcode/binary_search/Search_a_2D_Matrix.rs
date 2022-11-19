// https://leetcode.com/problems/search-a-2d-matrix/

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // Binary search in the first column. Then binary search in the row.
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }

        let mut left = 0 as i32;
        let mut right = (matrix.len() - 1) as i32;
        while left <= right {
            let mid = (left + right) / 2;
            match matrix[mid as usize][0].cmp(&target) {
                Ordering::Greater => {
                    right = mid - 1;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }

        if right < 0 {
            return false;
        }

        let row = &matrix[right as usize];
        let mut left = 0 as i32;
        let mut right = (row.len() - 1) as i32;

        while left <= right {
            let mid = (left + right) / 2;
            match row[mid as usize].cmp(&target) {
                Ordering::Greater => {
                    right = mid - 1;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
