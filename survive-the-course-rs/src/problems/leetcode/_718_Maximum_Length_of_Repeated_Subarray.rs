// https://leetcode.com/problems/maximum-length-of-repeated-subarray/description/

pub struct Solution1;
// TODO: Rolling-hash. Check the solution on leetcode
pub struct Solution2;

impl Solution1 {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut result = 0;
        for row_idx in (0..nums1.len()).rev() {
            for col_idx in (0..nums2.len()).rev() {
                if nums1[row_idx] == nums2[col_idx] {
                    dp[row_idx][col_idx] = dp[row_idx + 1][col_idx + 1] + 1;
                    result = result.max(dp[row_idx][col_idx]);
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
