// https://leetcode.com/problems/sliding-window-maximum/

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut left_pointer = 0;
        let mut right_pointer = 0;
        let mut q: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        while right_pointer < nums.len() {
            while q.len() > 0 && nums[*q.back().unwrap()] <= nums[right_pointer] {
                q.pop_back();
            }
            q.push_back(right_pointer);
            if right_pointer - left_pointer == k {
                left_pointer += 1;
                while q.len() > 0 && *q.front().unwrap() < left_pointer {
                    q.pop_front();
                }
            }
            if right_pointer >= k - 1 {
                result.push(nums[*q.front().unwrap()]);
            }
            right_pointer += 1;
        }
        for idx in 0..k {
            q.push_back(idx);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }
}
