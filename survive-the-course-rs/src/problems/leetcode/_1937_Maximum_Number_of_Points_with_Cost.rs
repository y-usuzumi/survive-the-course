// https://leetcode.com/problems/maximum-number-of-points-with-cost/

pub struct Solution;

// TODO: Add comments
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let rows = points.len();
        let cols = points[0].len();
        let mut result = vec![vec![0i64; cols]; rows];
        for col_idx in 0..cols {
            result[0][col_idx] = points[0][col_idx] as i64;
        }
        for row_idx in 1..rows {
            let mut forward_max = i64::MIN;
            for col_idx in 0..cols {
                let cell = points[row_idx][col_idx] as i64;
                let prev_cell = result[row_idx - 1][col_idx] as i64;
                forward_max = forward_max.max(prev_cell + col_idx as i64);
                result[row_idx][col_idx] =
                    result[row_idx][col_idx].max(cell + forward_max - col_idx as i64);
            }

            let mut backward_max = i64::MIN;
            for col_idx in (0..cols).rev() {
                let cell = points[row_idx][col_idx] as i64;
                let prev_cell = result[row_idx - 1][col_idx] as i64;
                backward_max = backward_max.max(prev_cell - col_idx as i64);
                result[row_idx][col_idx] =
                    result[row_idx][col_idx].max(cell + backward_max + col_idx as i64);
            }
        }
        return *result[result.len() - 1].iter().max().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]];
        assert_eq!(Solution::max_points(points), 9);
    }

    #[test]
    fn test_2() {
        let points = vec![
            vec![4, 1, 0, 4, 0],
            vec![1, 0, 4, 0, 5],
            vec![1, 3, 0, 4, 1],
            vec![4, 4, 0, 4, 0],
        ];
        assert_eq!(Solution::max_points(points), 15);
    }
}
