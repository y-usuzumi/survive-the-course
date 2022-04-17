use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut bag_of_chars = HashSet::new();
        let (mut idxl, mut idxr) = (0, 0);
        let mut curr_max = 0;
        let mut next_char;
        while idxr < chars.len() {
            next_char = chars[idxr];
            if bag_of_chars.contains(&next_char) {
                while idxl < idxr && idxl < chars.len() {
                    let left_char = chars[idxl];
                    bag_of_chars.remove(&left_char);
                    idxl += 1;
                    if left_char == next_char {
                        break;
                    }
                }
            }
            bag_of_chars.insert(next_char);
            idxr += 1;
            curr_max = std::cmp::max(idxr - idxl, curr_max);
        }
        curr_max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    }
}