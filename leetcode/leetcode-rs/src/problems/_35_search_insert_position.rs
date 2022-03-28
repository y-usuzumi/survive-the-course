struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right): (i32, i32) = (0, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                return mid as i32;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}