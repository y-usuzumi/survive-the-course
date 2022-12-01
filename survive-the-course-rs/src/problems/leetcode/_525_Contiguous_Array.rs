// https://leetcode.com/problems/contiguous-array/

use std::collections::{hash_map::Entry, HashMap};

pub struct Solution;

impl Solution {
    // Prefix sum + HashMap
    //
    // Iterating through the array from the left to the right
    // Use a counter that maintains the current number of 1's (or 0's using
    // a negative number) that are yet to be cancelled out.
    // For example:
    // [1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1]
    // The corresponding counter value for each position:
    // [1, 2, 3, 2, 1, 2, 1, 0, -1, -2, 1]
    // Then we need a hashmap to maintain the position of the previous occurrence
    // Of each counter value. For example:
    // {1: 0, 2: 4, 3: 2}
    // is read as: there is a position on the left of the current cursor where
    // the counter was 1, and the position is 0... etc
    // We use this counter so that when we run into a counter value that exists
    // before, we can simply measure the distance between these two positions,
    // and that range sums to zero.
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        hm.insert(0, 0);
        let mut counter = 0;

        for idx in 0..nums.len() {
            if nums[idx] == 1 {
                counter += 1;
            } else {
                counter -= 1;
            }

            match hm.entry(counter) {
                Entry::Vacant(e) => {
                    e.insert(idx + 1);
                }
                Entry::Occupied(mut e) => {
                    let val = e.get_mut();
                    max_length = max_length.max(idx + 1 - *val);
                }
            }
        }
        return max_length as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}
