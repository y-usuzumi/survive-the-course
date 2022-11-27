// https://leetcode.com/problems/find-the-duplicate-number/

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0 as usize;
        let mut fast = 0 as usize;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        let mut slow2 = 0 as usize;
        while slow2 != slow {
            slow2 = nums[slow2] as usize;
            slow = nums[slow] as usize;
        }

        return slow as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
