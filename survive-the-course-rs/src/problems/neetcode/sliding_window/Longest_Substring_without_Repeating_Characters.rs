// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::HashSet;

pub trait Solution {
    fn length_of_longest_substring(s: String) -> i32;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // Neetcode上的参考做法，使用Set
    fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut idxl = 0;
        let mut result = 0;
        let chars = s.chars().collect::<Vec<_>>();
        for ch in chars.iter() {
            if !set.contains(ch) {
                set.insert(ch);
                result = result.max(set.len());
            } else {
                // 这个其实比较慢
                while set.contains(ch) {
                    set.remove(&chars[idxl]);
                    idxl += 1;
                }
                set.insert(ch);
            }
        }
        return result as i32;
    }
}

impl Solution for Solution2 {
    // 我2019年用Python的做法
    fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut result = 0;
        let mut curr_len = 0;
        // The first char that is safe to include
        let mut head_idx = 0;
        let chars = s.chars().collect::<Vec<_>>();
        for (idx, ch) in chars.iter().enumerate() {
            match map.entry(*ch) {
                Occupied(mut e) => {
                    let last_shown_idx = e.get();
                    head_idx = head_idx.max(last_shown_idx + 1);
                    curr_len = (idx - head_idx) + 1;
                    result = result.max(curr_len);
                    *(e.get_mut()) = idx;
                }
                Vacant(e) => {
                    e.insert(idx);
                    curr_len += 1;
                    result = result.max(curr_len);
                }
            }
        }
        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution1::length_of_longest_substring("abcabcbb".to_string()),
                3
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution1::length_of_longest_substring("bbbbb".to_string()),
                1
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution1::length_of_longest_substring("pwwkew".to_string()),
                3
            );
        }
    }

    mod solution2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution2::length_of_longest_substring("abcabcbb".to_string()),
                3
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution2::length_of_longest_substring("bbbbb".to_string()),
                1
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution2::length_of_longest_substring("pwwkew".to_string()),
                3
            );
        }

        #[test]
        fn test_4() {
            assert_eq!(
                Solution2::length_of_longest_substring("tmmzuxt".to_string()),
                5
            );
        }

        #[test]
        fn test_5() {
            assert_eq!(
                Solution2::length_of_longest_substring("wobgrovw".to_string()),
                6
            );
        }
    }
}
