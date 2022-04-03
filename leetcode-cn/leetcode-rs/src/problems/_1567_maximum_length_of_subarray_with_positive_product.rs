// https://leetcode-cn.com/problems/maximum-length-of-subarray-with-positive-product/

use std::cmp::max;
use std::slice::Iter;

pub struct Solution;

impl Solution {
    pub fn get_max_length(nums: Vec<i32>) -> i32 {
        let max_length_forward = Self::max_length_unidirectional(nums.iter());
        let max_length_backward = Self::max_length_unidirectional(nums.iter().rev());
        max(max_length_backward, max_length_forward)
    }

    fn max_length_unidirectional(nums: Iter<i32>) -> i32 {
        let mut max_length = 0;
        let mut curr_length = 0;
        let mut pending_length = 0;
        let mut is_negs_odd = false;
        for num in nums.iter() {
            if *num == 0 {
                curr_length = 0;
                pending_length = 0;
                is_negs_odd = false;
            } else if *num < 0 {
                if is_negs_odd {
                    curr_length += pending_length + 1;
                    pending_length = 0;
                } else {
                    pending_length += 1;
                }
                is_negs_odd = !is_negs_odd;
            } else {
                if is_negs_odd {
                    pending_length += 1;
                } else {
                    curr_length += 1;
                }
            }
            if curr_length > max_length {
                max_length = curr_length;
            }
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_max_length(vec![1, -2, -3, 4]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_max_length(vec![0, 1, -2, -3, -4]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_max_length(vec![-1, -2, -3, 0, 1]), 2);
    }
}
