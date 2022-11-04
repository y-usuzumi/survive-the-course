// https://leetcode.cn/problems/longest-consecutive-sequence/
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution;

impl Solution {
    // 首先根据原数组生成Set,然后找到每个连续子数据的起始元素（因为起始元素-1不在Set之内）
    // 接着不断+1,直到数字不在Set之内，则找到了结束数字。找到后更新最大值。
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let s: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut longest = 0;
        for num in nums {
            if !s.contains(&(num - 1)) {
                let mut length = 1;
                while s.contains(&(num + length)) {
                    length += 1;
                }
                longest = longest.max(length);
            }
        }
        return longest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
