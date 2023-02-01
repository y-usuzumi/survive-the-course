// https://leetcode.com/problems/permutations-ii/

use std::collections::{hash_map::Entry, HashMap};

pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }

        let mut counter: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *counter.entry(num).or_default() += 1;
        }

        fn dfs(counter: &mut HashMap<i32, i32>, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if counter.is_empty() {
                result.push(stack.clone());
            }

            let keys: Vec<i32> = counter.keys().map(|k| *k).collect();
            for k in keys {
                stack.push(k);
                let entry = counter.entry(k);
                if let Entry::Occupied(mut e) = entry {
                    let ref_mut = e.get_mut();
                    *ref_mut -= 1;
                    if *ref_mut == 0 {
                        e.remove();
                    }
                }
                dfs(counter, stack, result);
                stack.pop();
                *counter.entry(k).or_default() += 1;
            }
        }
        let mut stack = vec![];
        let mut result = vec![];
        dfs(&mut counter, &mut stack, &mut result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use test_util::assert_eq_ignore_order;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq_ignore_order(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
        );
    }
}
