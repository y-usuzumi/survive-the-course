// https://leetcode.com/problems/maximum-subarray/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut curr_sum = 0;
        for num in nums {
            curr_sum += num;
            max = max.max(curr_sum);
            if curr_sum < 0 {
                curr_sum = 0;
                continue;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_2() {
        // Note: the problem is unclear. Empty subarrays are actually not allowed
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
