// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let mut result = 0;
        let mut longest_distance_map = vec![vec![-1; matrix[0].len()]; matrix.len()];

        for row_idx in 0..matrix.len() {
            for col_idx in 0..matrix[0].len() {
                result = result.max(Self::dfs(
                    row_idx,
                    col_idx,
                    &matrix,
                    &mut longest_distance_map,
                ));
            }
        }

        return result;
    }

    pub fn dfs(
        row_idx: usize,
        col_idx: usize,
        matrix: &Vec<Vec<i32>>,
        longest_distance_map: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if longest_distance_map[row_idx][col_idx] < 0 {
            let mut longest_distance = 0;
            if row_idx < matrix.len() - 1 && matrix[row_idx + 1][col_idx] < matrix[row_idx][col_idx]
            {
                longest_distance = longest_distance.max(Self::dfs(
                    row_idx + 1,
                    col_idx,
                    matrix,
                    longest_distance_map,
                ));
            }
            if col_idx < matrix[0].len() - 1
                && matrix[row_idx][col_idx + 1] < matrix[row_idx][col_idx]
            {
                longest_distance = longest_distance.max(Self::dfs(
                    row_idx,
                    col_idx + 1,
                    matrix,
                    longest_distance_map,
                ));
            }
            if row_idx > 0 && matrix[row_idx - 1][col_idx] < matrix[row_idx][col_idx] {
                longest_distance = longest_distance.max(Self::dfs(
                    row_idx - 1,
                    col_idx,
                    matrix,
                    longest_distance_map,
                ));
            }
            if col_idx > 0 && matrix[row_idx][col_idx - 1] < matrix[row_idx][col_idx] {
                longest_distance = longest_distance.max(Self::dfs(
                    row_idx,
                    col_idx - 1,
                    matrix,
                    longest_distance_map,
                ));
            }
            longest_distance += 1;
            longest_distance_map[row_idx][col_idx] = longest_distance;
        }

        return longest_distance_map[row_idx][col_idx];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }
}
