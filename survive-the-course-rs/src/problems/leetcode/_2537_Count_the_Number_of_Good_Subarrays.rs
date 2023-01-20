// https://leetcode.com/problems/count-the-number-of-good-subarrays/description/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        if nums.is_empty() {
            return 0;
        }
        let mut h: HashMap<i32, i32> = HashMap::new();
        let mut good_pairs_count = 0;
        let mut result = 0;
        let mut left = 0;
        for num in nums.iter() {
            result += left;
            let prev_count_ref = h.entry(*num).or_default();
            good_pairs_count += *prev_count_ref;
            *prev_count_ref += 1;
            while good_pairs_count >= k {
                result += 1;
                left += 1;
                let rem_count_ref = h.get_mut(&nums[left - 1]).unwrap();
                *rem_count_ref -= 1;
                good_pairs_count -= *rem_count_ref;
            }
        }

        return result as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
