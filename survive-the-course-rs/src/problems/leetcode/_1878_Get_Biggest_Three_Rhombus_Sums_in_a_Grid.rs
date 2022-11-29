// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid/

pub struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        if grid.is_empty() || grid[0].is_empty() {
            return Vec::new();
        }
        // Prefix sum ↘
        let mut prefix_sum_p = vec![vec![0; grid[0].len()]; grid.len()];
        // Prefix sum ↗
        let mut prefix_sum_n = vec![vec![0; grid[0].len()]; grid.len()];
        for col_idx in 0..grid[0].len() {
            prefix_sum_p[0][col_idx] = grid[0][col_idx];
            prefix_sum_n[0][col_idx] = grid[0][col_idx];
        }
        for row_idx in 1..grid.len() {
            for col_idx in 0..grid[0].len() {
                if col_idx == 0 {
                    prefix_sum_p[row_idx][col_idx] = grid[row_idx][col_idx];
                } else {
                    prefix_sum_p[row_idx][col_idx] =
                        prefix_sum_p[row_idx - 1][col_idx - 1] + grid[row_idx][col_idx];
                }
                if col_idx == grid[0].len() - 1 {
                    prefix_sum_n[row_idx][col_idx] = grid[row_idx][col_idx];
                } else {
                    prefix_sum_n[row_idx][col_idx] =
                        prefix_sum_n[row_idx - 1][col_idx + 1] + grid[row_idx][col_idx];
                }
            }
        }

        let mut result = Vec::new();

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                Self::merge_result(&mut result, grid[row_idx][col_idx]);
                let mut size = 1;
                while row_idx >= size
                    && row_idx + size < grid.len()
                    && col_idx + 2 * size < grid[0].len()
                {
                    let (lr, lc) = (row_idx, col_idx);
                    let (tr, tc) = (row_idx - size, col_idx + size);
                    let (br, bc) = (row_idx + size, col_idx + size);
                    let (rr, rc) = (row_idx, col_idx + 2 * size);
                    let mut rhombus_sum = 0;
                    rhombus_sum += prefix_sum_n[lr][lc] - prefix_sum_n[tr][tc];
                    rhombus_sum += prefix_sum_p[rr][rc] - prefix_sum_p[tr][tc];
                    rhombus_sum += prefix_sum_p[br][bc] - prefix_sum_p[lr][lc];
                    rhombus_sum += prefix_sum_n[br][bc] - prefix_sum_n[rr][rc];
                    rhombus_sum += grid[tr][tc];
                    rhombus_sum -= grid[br][bc];
                    Self::merge_result(&mut result, rhombus_sum);
                    size += 1;
                }
            }
        }

        result.reverse();
        return result;
    }

    fn merge_result(result: &mut Vec<i32>, rhombus_sum: i32) {
        if result.contains(&rhombus_sum) {
            return;
        }
        if result.len() < 3 {
            result.push(rhombus_sum);
            result.sort();
            return;
        }
        if rhombus_sum < result[0] {
            return;
        }

        if rhombus_sum < result[1] {
            result[0] = rhombus_sum;
        } else if rhombus_sum < result[2] {
            result[0] = result[1];
            result[1] = rhombus_sum;
        } else {
            result[0] = result[1];
            result[1] = result[2];
            result[2] = rhombus_sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 3, 4, 2, 3],
            vec![20, 30, 200, 40, 10],
            vec![1, 5, 5, 4, 1],
            vec![4, 3, 2, 2, 5],
        ];
        assert_eq!(Solution::get_biggest_three(grid), vec![228, 216, 211]);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::get_biggest_three(grid), vec![20, 9, 8]);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]];
        assert_eq!(Solution::get_biggest_three(grid), vec![2, 1]);
    }
}
