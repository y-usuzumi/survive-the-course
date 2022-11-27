// https://leetcode.com/problems/first-missing-positive/

use std::mem;

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for idx in 0..nums.len() {
            while nums[idx] > 0
                && nums[idx] < nums.len() as i32
                && nums[idx] != idx as i32 + 1
                && nums[nums[idx] as usize - 1] != nums[idx]
            {
                let target_idx = nums[idx];
                nums[idx] = nums[target_idx as usize - 1];
                nums[target_idx as usize - 1] = target_idx;
            }
        }

        for idx in 0..nums.len() {
            if nums[idx] != idx as i32 + 1 {
                return idx as i32 + 1;
            }
        }

        return nums.len() as i32 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::first_missing_positive(vec![2, 1]), 3);
    }
}
