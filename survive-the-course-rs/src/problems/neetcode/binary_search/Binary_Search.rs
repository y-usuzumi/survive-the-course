// https://leetcode.com/problems/binary-search/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let mid = right - (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Less => {
                    left = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    right = mid - 1;
                }
                _ => {
                    return mid as i32;
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
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }
}
