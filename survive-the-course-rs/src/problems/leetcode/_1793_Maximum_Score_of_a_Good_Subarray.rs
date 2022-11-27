// https://leetcode.com/problems/maximum-score-of-a-good-subarray/

pub trait Solution {
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32;
}

pub struct Solution1;

pub struct Solution2;

pub struct Solution3;

impl Solution for Solution1 {
    // 1. Generate min values for all numbers
    // 2. Double-pointer. Shift on the larger side
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut mins = vec![0; nums.len()];
        let k = k as usize;
        mins[k] = nums[k];
        {
            let mut curr_min = nums[k];
            for idx in (0..k).rev() {
                if nums[idx] < curr_min {
                    curr_min = nums[idx];
                }
                mins[idx] = curr_min;
            }
        }
        {
            let mut curr_min = nums[k];
            for idx in k + 1..nums.len() {
                if nums[idx] < curr_min {
                    curr_min = nums[idx];
                }
                mins[idx] = curr_min;
            }
        }
        let mut i = k;
        let mut j = k;
        let mut max_score = nums[k];
        while j < nums.len() {
            let min_i = mins[i];
            let min_j = mins[j];
            let curr_score = min_i.min(min_j) * (j + 1 - i) as i32;
            max_score = max_score.max(curr_score);
            if i == 0 {
                j += 1;
            } else if j == nums.len() - 1 {
                i -= 1;
            } else if mins[i - 1] < mins[j + 1] {
                j += 1;
            } else {
                i -= 1;
            }
        }
        return max_score;
    }
}

impl Solution for Solution2 {
    // We don't even need a min table. We can simply update the current left and
    // right min values on the fly.
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut i = k;
        let mut j = k;
        let mut curr_min_i = nums[k];
        let mut curr_min_j = nums[k];
        let mut max_score = nums[k];
        while j < nums.len() {
            curr_min_i = curr_min_i.min(nums[i]);
            curr_min_j = curr_min_j.min(nums[j]);
            let curr_score = curr_min_i.min(curr_min_j) * (j + 1 - i) as i32;
            max_score = max_score.max(curr_score);
            if i == 0 {
                j += 1;
            } else if j == nums.len() - 1 {
                i -= 1;
            } else {
                if curr_min_i.min(nums[i - 1]) < curr_min_j.min(nums[j + 1]) {
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
        return max_score;
    }
}

impl Solution for Solution3 {
    // Sparse table: https://www.geeksforgeeks.org/sparse-table/
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution1::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution1::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0),
                20
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution1::maximum_score(
                    vec![6569, 9667, 3148, 7698, 1622, 2194, 793, 9041, 1670, 1872],
                    5
                ),
                9732
            );
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution2::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution2::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0),
                20
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution2::maximum_score(
                    vec![6569, 9667, 3148, 7698, 1622, 2194, 793, 9041, 1670, 1872],
                    5
                ),
                9732
            );
        }
    }
}
