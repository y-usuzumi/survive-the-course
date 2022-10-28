// https://leetcode.com/problems/valid-anagram/
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub trait Solution {
    fn is_anagram(s: String, t: String) -> bool;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // 从s创建counter,然后对每个t中的char,从counter中减去，
    // 最后counter应为空。
    fn is_anagram(s: String, t: String) -> bool {
        let mut counter = HashMap::new();

        for ch in s.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            match counter.entry(ch) {
                Entry::Occupied(mut entry) => {
                    let val = *entry.get();
                    if val == 1 {
                        entry.remove();
                    } else {
                        *entry.get_mut() -= 1;
                    }
                }
                Entry::Vacant(_entry) => return false,
            }
        }
        return counter.is_empty();
    }
}

impl Solution for Solution2 {
    // 排序并比较
    fn is_anagram(s: String, t: String) -> bool {
        let mut schars = s.chars().collect::<Vec<_>>();
        let mut tchars = t.chars().collect::<Vec<_>>();
        schars.sort();
        tchars.sort();
        return schars == tchars;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            let s = "anagram".to_string();
            let t = "nagaram".to_string();
            assert!(Solution1::is_anagram(s, t));
        }

        #[test]
        fn test_2() {
            let s = "anagram".to_string();
            let t = "nagarm".to_string();
            assert!(!Solution1::is_anagram(s, t));
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            let s = "anagram".to_string();
            let t = "nagaram".to_string();
            assert!(Solution2::is_anagram(s, t));
        }

        #[test]
        fn test_2() {
            let s = "anagram".to_string();
            let t = "nagarm".to_string();
            assert!(!Solution2::is_anagram(s, t));
        }
    }
}
