// https://leetcode.com/problems/jump-game-ii/

pub struct Solution;

impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums[len - 1] = 0;
        for idx in (0..nums.len() - 1).rev() {
            let mut min_steps = i32::MAX - 1;
            for idx2 in 1..=nums[idx] as usize {
                if idx + idx2 >= len {
                    break;
                }
                min_steps = min_steps.min(nums[idx + idx2]);
            }
            nums[idx] = min_steps + 1;
        }
        return nums[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::jump(vec![1, 1, 1, 1]), 3);
    }
}
