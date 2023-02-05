// https://leetcode.com/problems/find-all-anagrams-in-a-string/

pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.is_empty() {
            return vec![];
        }

        #[inline]
        fn char_to_idx(ch: char) -> usize {
            return (ch as u8 - 'a' as u8) as usize;
        }

        let mut target = vec![0; 26];
        for ch in p.chars() {
            target[char_to_idx(ch)] += 1;
        }
        let mut result = vec![];
        let mut count = vec![0; 26];
        let mut l = 0;
        let mut r = 0;
        let schars: Vec<char> = s.chars().collect();
        while r < schars.len() {
            let char_idx = char_to_idx(schars[r]);
            count[char_idx] += 1;
            if count == target {
                result.push(l as i32);
                count[char_to_idx(schars[l])] -= 1;
                l += 1;
            }
            while l <= r && count[char_idx] > target[char_idx] {
                count[char_to_idx(schars[l])] -= 1;
                l += 1;
            }
            r += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
