// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/

use std::collections::{hash_map::Entry, HashMap};

pub struct Solution;

// TODO: Add comments
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut xor_map: HashMap<i32, Vec<usize>> = HashMap::new();
        xor_map.insert(0, vec![0]);
        let mut xor_val = 0;
        for (idx, num) in arr.iter().enumerate() {
            xor_val ^= num;
            match xor_map.entry(xor_val) {
                Entry::Vacant(e) => {
                    e.insert(vec![idx + 1]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(idx + 1);
                }
            }
        }

        let mut result = 0;

        for (k, v) in xor_map {
            if v.len() < 2 {
                continue;
            }

            for idxidx1 in 0..v.len() - 1 {
                let idx1 = v[idxidx1];
                for idxidx2 in idxidx1..v.len() {
                    let idx2 = v[idxidx2];
                    if idx2 - idx1 < 2 {
                        continue;
                    }
                    result += idx2 - idx1 - 1;
                }
            }
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 0);
    }
}
