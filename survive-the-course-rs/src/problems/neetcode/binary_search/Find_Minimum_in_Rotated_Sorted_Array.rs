// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
pub struct Solution;

impl Solution {
    // If the mid value is greater than the rightmost value, it is in the left sorted
    // portion, and we need to binary search inside the right half.
    // On the other hand, if the mid value is less than the rightmost value, it is
    // in the right sorted portion, and we need to binary search inside the left half.
    // Be careful that the mid value may just be the minimum value, and we should
    // update the current minimum value
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        let mut result = nums[right - 1];
        while left < right {
            let mid = (left + right - 1) / 2;
            if nums[mid] > nums[right - 1] {
                left = mid + 1;
            } else {
                // The
                result = result.min(nums[mid]);
                right = mid;
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
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
    }
}
