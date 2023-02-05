// https://leetcode.com/problems/split-array-largest-sum/

pub trait Solution {
    fn split_array(nums: Vec<i32>, k: i32) -> i32;
}

// 2D DP
pub struct Solution1;
// Binary search
pub struct Solution2;

impl Solution for Solution1 {
    fn split_array(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut memo = vec![vec![i32::MAX; k as usize]; nums.len()];
        fn dp(memo: &mut Vec<Vec<i32>>, nums: &Vec<i32>, start_idx: usize, k: usize) -> i32 {
            if nums.len() - start_idx < k {
                return i32::MAX;
            }

            if memo[start_idx][k - 1] < i32::MAX {
                return memo[start_idx][k - 1];
            }

            if k == 1 {
                let result = nums[start_idx..].iter().sum();
                memo[start_idx][0] = result;
                return result;
            }

            let mut sum = 0;
            let mut result = i32::MAX;
            for idx in start_idx..nums.len() {
                sum += nums[idx];
                let mut subresult = dp(memo, nums, idx + 1, k - 1);
                if sum > subresult {
                    result = result.min(sum);
                    break;
                } else {
                    subresult = subresult.max(sum);
                    result = result.min(subresult);
                }
            }
            memo[start_idx][k - 1] = result;
            return result;
        }
        return dp(&mut memo, &nums, 0, k as usize);
    }
}

impl Solution for Solution2 {
    fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        fn helper(nums: &Vec<i32>, k: i32, max_sum: i32) -> bool {
            let mut subarr_count = 0;
            let mut sum = 0;
            for &num in nums {
                if sum + num > max_sum {
                    subarr_count += 1;
                    if subarr_count >= k {
                        return false;
                    }
                    sum = num;
                } else {
                    sum += num;
                }
            }
            return subarr_count < k;
        }

        let mut max_num = i32::MIN;
        let mut sum = 0;
        for &num in &nums {
            max_num = max_num.max(num);
            sum += num;
        }

        let mut left = max_num;
        let mut right = sum;

        while left < right {
            let mid = left + (right - left) / 2;
            if helper(&nums, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
