// https://leetcode.com/problems/interleaving-string/description/

pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.is_empty() {
            return s2 == s3;
        }
        if s2.is_empty() {
            return s1 == s3;
        }
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let chars3: Vec<char> = s3.chars().collect();
        if chars3.len() != chars1.len() + chars2.len() {
            return false;
        }
        // Rows represent number of chars we are using up to in s1.
        // Cols represent number of chars we are using up to in s2.
        // Row 0 / Col 0 means we do not use any of characters in that string.
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        // An empty string is an interleaving string from two empty strings.
        dp[0][0] = true;
        // Initialize the first row/col, meaning chars in s3 only come from the
        // one string.
        for idx in 0..s1.len() {
            if chars1[idx] != chars3[idx] {
                break;
            }
            dp[idx + 1][0] = true;
        }
        for idx in 0..s2.len() {
            if chars2[idx] != chars3[idx] {
                break;
            }
            dp[0][idx + 1] = true;
        }

        for idx1 in 1..=s1.len() {
            for idx2 in 1..=s2.len() {
                // Ex: Imagine we are populating dp[2][4], how do we know if the
                // first 2 chars from s1 and 4 chars from s2 make an
                // interleaving string of (2 + 4 = 6) chars from s3? Either
                // dp[2][3] is true, so we can append the 4th char from s2 to
                // the string, or dp[1][4] is true, so we can append the 2nd
                // char from s1 to the string.
                if dp[idx1 - 1][idx2] && chars3[idx1 + idx2 - 1] == chars1[idx1 - 1]
                    || dp[idx1][idx2 - 1] && chars3[idx1 + idx2 - 1] == chars2[idx2 - 1]
                {
                    dp[idx1][idx2] = true;
                }
            }
        }

        return dp[dp.len() - 1][dp[0].len() - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!()
    }
}
