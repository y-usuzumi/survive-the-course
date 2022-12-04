// https://leetcode.com/problems/subarray-sums-divisible-by-k/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // Theorem: For two positions in nums, if their prefix-sum-modulo-k are the
    // equal, then the two positions define a subarray that sums up to a number
    // which modulo k equals 0.
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        hm.insert(0, 1);
        let mut result = 0;
        let mut sum = 0;
        for num in nums {
            sum += num;
            sum = sum.rem_euclid(k);
            if hm.contains_key(&sum) {
                let val = hm.get_mut(&sum).unwrap();
                result += *val;
                *val += 1;
            } else {
                hm.insert(sum, 1);
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
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
}
