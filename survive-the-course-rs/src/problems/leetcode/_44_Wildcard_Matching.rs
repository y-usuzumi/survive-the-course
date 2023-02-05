// https://leetcode.com/problems/wildcard-matching/

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let schars: Vec<char> = s.chars().collect();
        let pchars: Vec<char> = p.chars().collect();

        fn is_match(schar: char, pchar: char) -> bool {
            return pchar == '?' || schar == pchar;
        }

        fn helper(
            dp: &mut Vec<Vec<Option<bool>>>,
            s: &Vec<char>,
            p: &Vec<char>,
            sidx: usize,
            pidx: usize,
        ) -> bool {
            if pidx >= p.len() {
                return sidx >= s.len();
            }

            if let Some(result) = dp[sidx][pidx] {
                return result;
            }

            let mut result = false;
            if p[pidx] == '*' {
                for idx in sidx..=s.len() {
                    result = helper(dp, s, p, idx, pidx + 1);
                    if result {
                        break;
                    }
                }
            } else if sidx < s.len() && is_match(s[sidx], p[pidx]) {
                result = helper(dp, s, p, sidx + 1, pidx + 1);
            } else {
                result = false;
            }
            dp[sidx][pidx] = Some(result);
            return result;
        }

        let mut dp = vec![vec![None; pchars.len() + 1]; schars.len() + 1];
        return helper(&mut dp, &schars, &pchars, 0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        Solution::is_match("acdcb".to_string(), "a*c?b".to_string());
    }
}
