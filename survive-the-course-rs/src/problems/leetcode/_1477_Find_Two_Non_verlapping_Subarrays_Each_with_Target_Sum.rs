// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/

pub struct Solution;

impl Solution {
    // Sliding window + DP
    // Sliding window to find subarrays that sum to target
    // DP to keep track of the min subarray up to left pointer - 1.
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        if arr.len() < 2 {
            return -1;
        }

        let mut min_sum_size = usize::MAX;
        let mut min_to_pos = vec![usize::MAX; arr.len()];

        let mut l = 0;
        let mut r = 0;
        let mut curr_sum = arr[l];
        loop {
            if curr_sum < target {
                if r > 0 {
                    min_to_pos[r] = min_to_pos[r - 1];
                }

                r += 1;
                if r == arr.len() {
                    break;
                }
                curr_sum += arr[r];
            } else if curr_sum > target {
                curr_sum -= arr[l];
                l += 1;
            } else {
                if l > 0 && min_to_pos[l - 1] < usize::MAX {
                    min_sum_size = min_sum_size.min(min_to_pos[l - 1] + r - l + 1);
                }

                if r > 0 {
                    min_to_pos[r] = min_to_pos[r - 1].min(r - l + 1);
                } else {
                    min_to_pos[r] = 1;
                }

                r += 1;
                if r == arr.len() {
                    break;
                }
                curr_sum += arr[r];
                curr_sum -= arr[l];
                l += 1;
            }
        }
        return if min_sum_size == usize::MAX {
            -1
        } else {
            min_sum_size as i32
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
            -1
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![1, 2, 2, 3, 2, 6, 7, 2, 1, 4, 8], 5),
            4
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![31, 1, 1, 18, 15, 3, 15, 14], 33),
            5
        );
    }
}
