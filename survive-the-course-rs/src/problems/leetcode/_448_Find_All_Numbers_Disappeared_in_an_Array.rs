// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/

pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for idx in 0..nums.len() {
            while nums[idx] != idx as i32 + 1 {
                let target_idx = nums[idx] as usize - 1;
                if nums[idx] == nums[target_idx] {
                    break;
                }
                let temp = nums[idx];
                nums[idx] = nums[target_idx];
                nums[target_idx] = temp;
            }
        }

        let mut result = Vec::with_capacity(nums.len());
        for idx in 0..nums.len() {
            if nums[idx] != idx as i32 + 1 {
                result.push(idx as i32 + 1);
            }
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
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
