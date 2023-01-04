// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/

pub struct Solution;

impl Solution {
    // We only need iterate over nums for one run.
    // First, we mark the start point as left_end
    // Once we find both lower_bound and upper_bound, we have found
    // (lower_bound - left_end + 1) valid subarrays. As we continue looking at
    // nums, as long as they are between the bounds, we found another
    // (lower_bound - left_end + 1) valid subarrays.
    //
    // Ex: min_k = 3, max_k = 10, nums = [1, 3, 10, 5, 8]
    // First, we set left_end = 1. Then we see 3 which is the lower bound, we
    // set lower_bound_idx = 1. Then we see 10 which is the upper bound, we
    // set upper_bound_idx = 2. Now the valid subarrays are: [1, 3, 10] and [3, 10].
    // Next, we see 5. Since lower_bound_idx and upper_bound_idx didn't change,
    // the new valid subarrays are: [1, 3, 10, 5] and [3, 10, 5]. And we keep
    // going.
    //
    // Whenever we meet a number beyond the bounds, we reset left_end and both
    // bounds.
    //
    // Another detail is when we meet another number equal to either of the
    // bounds, we need to update the bound_idx. For example, if we have
    // nums = [1, 3, 10, 5, 3, 8], when we meet the second 3, we need to update
    // lower_bound_idx to 4 so the shortest valid subarray is [10, 5, 3]. Then
    // we can include more subarrays: [1, 3, 10, 5, 3], [3, 10, 5, 3] and
    // [10, 5, 3].
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut left_end = 0usize;
        let mut lower_bound_idx = None;
        let mut upper_bound_idx = None;
        let mut result = 0;

        for (idx, num) in nums.into_iter().enumerate() {
            if num < min_k || num > max_k {
                left_end = idx + 1;
                lower_bound_idx = None;
                upper_bound_idx = None;
                continue;
            }
            if num == min_k {
                lower_bound_idx = Some(idx);
            }
            if num == max_k {
                upper_bound_idx = Some(idx);
            }

            if let (Some(lb), Some(ub)) = (lower_bound_idx, upper_bound_idx) {
                result += lb.min(ub) - left_end + 1;
            }
        }

        return result as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!()
    }
}
