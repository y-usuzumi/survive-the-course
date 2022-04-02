// https://leetcode-cn.com/problems/maximum-product-subarray/submissions/

use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut curr_neg_max, mut curr_pos_max) = (0, 0);
        if nums[0] > 0 {
            curr_pos_max = nums[0];
        } else {
            curr_neg_max = nums[0];
        }
        let mut max_product = nums[0];

        for idx in 1..nums.len() {
            let num = nums[idx];
            if num > 0 {
                curr_pos_max = max(num, curr_pos_max * num);
                curr_neg_max = curr_neg_max * num;
                
            } else {
                let (pm, nm) = (curr_pos_max, curr_neg_max);
                curr_neg_max = min(num, pm * num);
                curr_pos_max = nm * num;
                
            }
            max_product = max(max_product, curr_pos_max);
        }
        max_product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_product(vec![-2,0,-1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_product(vec![7,-2,-4]), 56);
    }
}