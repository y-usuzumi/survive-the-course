// https://leetcode.com/problems/next-permutation/

pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 1 {
            return;
        }

        let mut boundl = 0;
        let boundr = nums.len() - 1;
        'outer: for idx in (0..nums.len() - 1).rev() {
            if nums[idx] >= nums[idx + 1] {
                continue;
            }
            for idx2 in idx + 1..nums.len() {
                if idx2 == nums.len() - 1 || nums[idx2 + 1] <= nums[idx] {
                    nums.swap(idx, idx2);
                    boundl = idx + 1;
                    break 'outer;
                }
            }
        }
        Self::flip(nums, boundl, boundr);
    }

    fn flip(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr = vec![1, 2, 3];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut arr = vec![3, 2, 1];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut arr = vec![1, 1, 5];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 5, 1]);
    }

    #[test]
    fn test_4() {
        let mut arr = vec![1, 5, 1];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![5, 1, 1]);
    }

    #[test]
    fn test_5() {
        let mut arr = vec![1, 2, 6, 5, 4];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 4, 2, 5, 6]);
    }
}
