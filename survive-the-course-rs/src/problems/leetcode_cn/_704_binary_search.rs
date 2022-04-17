pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = (left + right) / 2;
            let num_mid = nums[mid as usize];
            if num_mid < target {
                left = mid + 1;
            } else if num_mid > target {
                right = mid - 1;
            } else {
                return mid as i32;
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
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_if_overflow() {
        let nums = vec![5];
        let target = -5;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
