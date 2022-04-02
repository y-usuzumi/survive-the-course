use std::cmp;

pub struct Solution;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut curr_max_minus_1, mut curr_max) = (nums[0], cmp::max(nums[0], nums[1]));
        let (mut curr_max_minus_1_start1, mut curr_max_start1) = (0, nums[1]);
        for idx in 2..nums.len()-1 {
            let mut next_max = cmp::max(curr_max_minus_1 + nums[idx], curr_max);
            curr_max_minus_1 = curr_max;
            curr_max = next_max;
            let mut next_max_start1 = cmp::max(curr_max_minus_1_start1 + nums[idx], curr_max_start1);
            curr_max_minus_1_start1 = curr_max_start1;
            curr_max_start1 = next_max_start1;
        }
        cmp::max(curr_max, curr_max_minus_1_start1+nums[nums.len()-1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(vec![2,3,2]), 3);
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
    }
}