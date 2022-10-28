// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

pub trait Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // Time complexity: O(nlogn)
    // Space complexity: O(1)
    fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        if nums.len() < 2 {
            return false;
        }
        let mut n1 = 0;
        let mut n2 = 1;
        while n2 < nums.len() {
            if nums[n1] == nums[n2] {
                return true;
            }
            n1 += 1;
            n2 += 1;
        }
        return false;
    }
}

impl Solution for Solution2 {
    // Time complexity: O(n)
    // Space complexity: O(n)
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut distinct_nums = HashSet::new();
        for num in nums.iter() {
            if distinct_nums.contains(num) {
                return true;
            }
            distinct_nums.insert(num);
        }
        return false;
    }
}
