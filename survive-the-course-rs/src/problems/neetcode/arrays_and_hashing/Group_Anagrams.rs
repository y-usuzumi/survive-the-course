// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars = vec![0; 26];
            for ch in s.chars() {
                chars[(ch as u32 - 'a' as u32) as usize] += 1;
            }
            m.entry(chars).or_default().push(s);
        }
        m.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use test_util::assert_eq_ignore_order;

    use super::*;

    #[test]
    fn test_1() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let output: Vec<Vec<String>> =
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
                .iter()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .collect();
        assert_eq_ignore_order(Solution::group_anagrams(input), output);
    }
}
