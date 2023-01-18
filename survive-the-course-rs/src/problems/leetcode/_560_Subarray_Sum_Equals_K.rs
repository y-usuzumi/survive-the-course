// https://leetcode.com/problems/subarray-sum-equals-k/description/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = HashMap::new();
        h.insert(0, 1);
        let mut curr_sum = 0;
        let mut result = 0;
        for num in nums {
            curr_sum += num;
            let diff = curr_sum - k;
            result += *h.entry(diff).or_default();
            *h.entry(curr_sum).or_default() += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
