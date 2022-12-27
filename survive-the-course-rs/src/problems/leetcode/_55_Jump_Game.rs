// https://leetcode.com/problems/jump-game/

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut curr_max_reach = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            if idx > curr_max_reach {
                return false;
            }
            curr_max_reach = curr_max_reach.max(num as usize + idx);
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::function(), 0);
    }
}
