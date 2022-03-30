use std::collections::{HashMap};
use std::iter::FromIterator;

struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            match hm.get(&(target - num)) {
                None => {
                    hm.insert(*num, idx);
                },
                Some(idx2) => {
                    return vec![*idx2 as i32, idx as i32];
                }
            }
        }
        panic!("Impossible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut result = Solution::two_sum(vec![2, 7, 11, 5], 9);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let mut result = Solution::two_sum(vec![3, 2, 4], 6);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let mut result = Solution::two_sum(vec![3, 3], 6);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }
}
