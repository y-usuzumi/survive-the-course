use std::cmp;

struct Solution;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut curr_max_minus_1 = nums[0];
        let mut curr_max = cmp::max(nums[0], nums[1]);
        for idx in 2..nums.len() {
            let next_max = cmp::max(curr_max_minus_1 + nums[idx], curr_max);
            curr_max_minus_1 = curr_max;
            curr_max = next_max;
        }
        curr_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
        assert_eq!(Solution::rob(vec![2,7,9,3,1]), 12);
    }
}