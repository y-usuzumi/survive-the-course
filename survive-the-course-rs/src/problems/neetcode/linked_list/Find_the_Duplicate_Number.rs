// https://leetcode.com/problems/find-the-duplicate-number/

pub struct Solution;

impl Solution {
    // Floyd's cycle finding algorithm (or Hare-tortoise algorithm)
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut tortoise = 0;
        let mut hare = 0;
        loop {
            tortoise = nums[tortoise] as usize;
            hare = nums[nums[hare] as usize] as usize;

            if tortoise == hare {
                break;
            }
        }

        let mut tortoise2 = 0;
        while tortoise2 != tortoise {
            tortoise = nums[tortoise] as usize;
            tortoise2 = nums[tortoise] as usize;
        }

        return tortoise as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
