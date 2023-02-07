// https://leetcode.com/problems/spiral-matrix-ii/

pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut curr = 1;
        let mut result = vec![vec![0; n]; n];
        let mut level = 0;
        while level < n / 2 {
            for col_idx in level..n - level - 1 {
                result[level][col_idx] = curr;
                curr += 1;
            }
            for row_idx in level..n - level - 1 {
                result[row_idx][n - level - 1] = curr;
                curr += 1;
            }
            for col_idx in (level + 1..n - level).rev() {
                result[n - level - 1][col_idx] = curr;
                curr += 1;
            }
            for row_idx in (level + 1..n - level).rev() {
                result[row_idx][level] = curr;
                curr += 1;
            }
            level += 1;
        }

        if n % 2 == 1 {
            result[n / 2][n / 2] = curr;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
