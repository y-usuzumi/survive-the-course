// https://leetcode.com/problems/valid-palindrome/

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            let lchar = chars[l];
            if !(lchar >= 'a' && lchar <= 'z')
                && !(lchar >= 'A' && lchar <= 'Z')
                && !(lchar >= '0' && lchar <= '9')
            {
                l += 1;
                continue;
            }

            let rchar = chars[r];
            if !(rchar >= 'a' && rchar <= 'z')
                && !(rchar >= 'A' && rchar <= 'Z')
                && !(rchar >= '0' && rchar <= '9')
            {
                r -= 1;
                continue;
            }
            if lchar.to_uppercase().next() != rchar.to_uppercase().next() {
                return false;
            }
            l += 1;
            r -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }
}
