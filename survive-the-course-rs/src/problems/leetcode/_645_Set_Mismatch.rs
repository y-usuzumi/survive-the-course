// https://leetcode.com/problems/set-mismatch/

pub struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut twice = -1;
        let mut missing = -1;
        for idx in 0..nums.len() {
            let target_num = nums[idx].abs() as usize;
            if nums[target_num - 1] < 0 {
                twice = target_num as i32;
            } else {
                nums[target_num - 1] = -nums[target_num - 1];
            }
        }

        for idx in 0..nums.len() {
            if nums[idx] > 0 {
                missing = idx as i32 + 1;
            }
        }

        vec![twice, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), [2, 3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), [1, 2]);
    }
}
