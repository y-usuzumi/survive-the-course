// https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/description/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut s1 = vec![];
        let mut s2 = vec![];

        fn helper(s1: &mut Vec<char>, s2: &mut Vec<char>, s: &Vec<char>, idx: usize) -> i32 {
            let mut curr_val = pal_val(s1) * pal_val(s2);
            if idx < s.len() {
                s1.push(s[idx]);
                curr_val = curr_val.max(helper(s1, s2, s, idx + 1));
                s1.pop();
                s2.push(s[idx]);
                curr_val = curr_val.max(helper(s1, s2, s, idx + 1));
                s2.pop();
                curr_val = curr_val.max(helper(s1, s2, s, idx + 1));
            }
            return curr_val;
        }

        fn pal_val(s: &Vec<char>) -> i32 {
            if s.is_empty() {
                return 0;
            }
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                if s[left] != s[right] {
                    return 1;
                }
                left += 1;
                right -= 1;
            }
            return s.len() as i32;
        }
        return helper(&mut s1, &mut s2, &chars, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
