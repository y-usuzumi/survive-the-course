// https://leetcode.com/problems/permutation-in-string/

pub struct Solution;

const ASCII_LOWERCASE_A: u32 = 97;

impl Solution {
    // The problem states that `s1` and `s2` consist of only lowercase English letters
    // 1. Use two counters of the 26 letters for s1 and s2.
    // 2. Use a variable to store the matched number of letters. Whenever it reads 26,
    //    returns true.
    // 3. Use two pointer in s2 to keep the size bounded.
    // 4. While moving the pointers, update both the counters and the variable.
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        #[inline]
        fn get_char_idx(ch: char) -> usize {
            return (ch as u32 - ASCII_LOWERCASE_A) as usize;
        }

        let mut s1_counters = [0; 26];
        let mut s2_counters = [0; 26];
        let mut matched_letters = 0;
        let s1chars: Vec<char> = s1.chars().collect();
        let s2chars: Vec<char> = s2.chars().collect();
        for idx in 0..s1chars.len() {
            s1_counters[get_char_idx(s1chars[idx])] += 1;
            s2_counters[get_char_idx(s2chars[idx])] += 1;
        }

        for i in 0..26 {
            if s1_counters[i] == s2_counters[i] {
                matched_letters += 1;
            }
        }

        if matched_letters == 26 {
            return true;
        }

        let mut left_pointer = 0;
        for right_pointer in s1chars.len()..s2chars.len() {
            let left_char_idx = get_char_idx(s2chars[left_pointer]);
            match s2_counters[left_char_idx] - s1_counters[left_char_idx] {
                0 => {
                    matched_letters -= 1;
                }
                1 => {
                    matched_letters += 1;
                }
                _ => {}
            }
            s2_counters[left_char_idx] -= 1;
            let right_char_idx = get_char_idx(s2chars[right_pointer]);
            match s1_counters[right_char_idx] - s2_counters[right_char_idx] {
                0 => {
                    matched_letters -= 1;
                }
                1 => {
                    matched_letters += 1;
                }
                _ => {}
            }
            s2_counters[right_char_idx] += 1;
            if matched_letters == 26 {
                return true;
            }
            left_pointer += 1;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ),);
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ),);
    }
}
