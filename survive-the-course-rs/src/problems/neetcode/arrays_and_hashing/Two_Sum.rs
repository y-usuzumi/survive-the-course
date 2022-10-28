// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // right operand -> index of left operand
        let mut m: HashMap<i32, usize> = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            if m.contains_key(num) {
                return vec![*m.get(num).unwrap() as i32, idx as i32];
            }
            m.insert(target - num, idx);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
