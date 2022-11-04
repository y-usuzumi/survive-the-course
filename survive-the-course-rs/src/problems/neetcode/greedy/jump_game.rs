// https://leetcode.com/problems/jump-game/

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut dead_zone_size = 0;
        for idx in (0..nums.len() - 1).rev() {
            if nums[idx] > dead_zone_size {
                dead_zone_size = 0;
                continue;
            }
            dead_zone_size += 1;
        }
        return dead_zone_size == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::can_jump(vec![0]));
    }
}
