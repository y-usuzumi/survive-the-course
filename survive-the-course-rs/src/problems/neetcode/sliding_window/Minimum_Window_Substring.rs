// https://leetcode.com/problems/minimum-window-substring/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 1. Create a counter map of the presence of each char in String `t`.
    // 2. Have two pointers at the start of String `s`.
    // 3. Also have a counter of matched chars.
    // 4. Move the right pointer, decrement the counter by one if it exists in the
    //    counter map. If it reaches zero, increment the counter of matched chars.
    // 5. Whenever matched chars == size of counter map, we can move the left pointer
    // 6. The left pointer may be too far left such that it includes more than needed
    //    number of certain chars, or it may include useless chars. Therefore, we can
    //    move it rightward. Make sure to increment the counter by one whenever a
    //    useful char is skipped, and decrement the counter of matched chars when
    //    the counter of the char in counter map goes back to greater than 1. At this point,
    //    we know that we need to accept more chars by moving the right pointer
    //    hoping that the missing char can be regained.
    pub fn min_window(s: String, t: String) -> String {
        let mut hm: HashMap<char, i32> = HashMap::new();
        for ch in t.chars() {
            *hm.entry(ch).or_default() += 1;
        }
        let chars_to_match = hm.len();

        let mut min_size = usize::MAX;
        let mut min_size_start_idx = 0 as usize;
        let mut matched = 0;

        let schars: Vec<char> = s.chars().collect();
        let mut left_idx = 0;
        for (right_idx, ch) in schars.iter().enumerate() {
            if hm.contains_key(ch) {
                let val = hm.get_mut(ch).unwrap();
                *val -= 1;
                if *val == 0 {
                    matched += 1;
                }
            }

            while matched == chars_to_match {
                let size = right_idx - left_idx + 1;
                if size < min_size {
                    min_size = size;
                    min_size_start_idx = left_idx;
                }
                let left_ch = schars[left_idx];
                if hm.contains_key(&left_ch) {
                    let val = hm.get_mut(&left_ch).unwrap();
                    *val += 1;
                    if *val == 1 {
                        matched -= 1;
                    }
                }
                left_idx += 1;
            }
        }

        if min_size == usize::MAX {
            return "".to_string();
        }

        return schars[min_size_start_idx..min_size_start_idx + min_size]
            .into_iter()
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(Solution::min_window(s, t), "BANC");
    }

    #[test]
    fn test_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        assert_eq!(Solution::min_window(s, t), "a");
    }

    #[test]
    fn test_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        assert_eq!(Solution::min_window(s, t), "");
    }
}
