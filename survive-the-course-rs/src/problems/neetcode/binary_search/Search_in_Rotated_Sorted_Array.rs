// https://leetcode.com/problems/search-in-rotated-sorted-array/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let leftusize = left as usize;
            let rightusize = right as usize;
            let mid = (leftusize + rightusize) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            // These condition checks take care of both cases when the array
            // is rotated and when it is not rotated.
            if nums[mid] >= nums[leftusize] {
                // That is the reason why the second branch (target < nums[leftusize])
                // cannot be replaced with (target <= nums[rightusize])
                if target > nums[mid] || target < nums[leftusize] {
                    left = mid as i32 + 1;
                } else {
                    right = mid as i32 - 1;
                }
            } else {
                if target < nums[mid] || target > nums[rightusize] {
                    right = mid as i32 - 1;
                } else {
                    left = mid as i32 + 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
    }
}
