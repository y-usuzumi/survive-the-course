// https://leetcode-cn.com/problems/permutation-in-string
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let mut baseline = HashMap::new();
        for ch in s1.chars() {
            *baseline.entry(ch).or_insert(0) += 1;
        }

        let mut compare = HashMap::new();
        for ch in s2.chars().take(s1.len()) {
            *compare.entry(ch).or_insert(0) += 1;
        }

        let s2chars: Vec<char> = s2.chars().collect();

        let mut left_bound = 0;
        let mut right_bound = s1.len();

        loop {
            if compare == baseline {
                return true;
            }
            if right_bound >= s2chars.len() {
                break;
            }

            let left_bound_char = s2chars[left_bound];
            *compare.get_mut(&left_bound_char).unwrap() -= 1;
            if compare[&left_bound_char] == 0 {
                compare.remove(&left_bound_char);
            }

            let right_bound_char = s2chars[right_bound];
            *compare.entry(right_bound_char).or_insert(0) += 1;
            left_bound += 1;
            right_bound += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }
}
