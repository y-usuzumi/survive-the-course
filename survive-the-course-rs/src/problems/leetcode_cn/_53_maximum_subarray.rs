// https://leetcode-cn.com/problems/maximum-subarray/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_max = i32::MIN;
        let mut curr_sum = 0;

        for num in nums {
            curr_sum = std::cmp::max(curr_sum + num, num);
            curr_max = std::cmp::max(curr_max, curr_sum);
        }
        curr_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}