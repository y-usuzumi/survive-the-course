// https://leetcode.com/problems/longest-mountain-in-array/

pub struct Solution;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        if arr.len() < 3 {
            return 0;
        }
        let mut result = 0;
        let mut mountain_left = 0;
        let mut mountain_top = usize::MAX;
        for idx in 1..arr.len() {
            if mountain_top == usize::MAX {
                if arr[idx] > arr[idx - 1] {
                    continue;
                } else if arr[idx] == arr[idx - 1] || idx - mountain_left < 2 {
                    mountain_left = idx;
                } else {
                    mountain_top = idx - 1;
                    result = result.max(idx - mountain_left + 1);
                }
            } else {
                if arr[idx] < arr[idx - 1] {
                    result = result.max(idx - mountain_left + 1);
                    continue;
                } else if arr[idx] > arr[idx - 1] {
                    mountain_top = usize::MAX;
                    mountain_left = idx - 1;
                } else {
                    mountain_top = usize::MAX;
                    mountain_left = idx;
                }
            }
        }
        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_mountain(vec![0, 1, 0]), 3);
    }
}
