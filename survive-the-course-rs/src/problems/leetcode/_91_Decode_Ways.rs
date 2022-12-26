// https://leetcode.com/problems/decode-ways/

pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 0 || chars[0] == '0' {
            return 0;
        }
        if chars.len() == 1 {
            return 1;
        }
        let mut dp = vec![0; chars.len() + 1];
        // Consider empty string is a solution so things like "12" can be made
        // to work with the recurrence equation as such:
        // dp("12") = dp("") + dp("1")
        dp[0] = 1;
        dp[1] = 1;
        for idx in 1..chars.len() {
            let curr_char = chars[idx] as u8;
            let prev_char = chars[idx - 1] as u8;
            if curr_char == b'0' {
                // "00", "30" shall not occur in a valid encoded string.
                if prev_char == b'0' || prev_char > b'2' {
                    return 0;
                }
                // The preceding digit needs to combine with 0 into a char.
                // So possible ways are the same as excluding the preceding digit.
                dp[idx + 1] = dp[idx - 1];
                continue;
            }
            // In all other cases, it is always possible that the current digit
            // makes a letter by itself.
            dp[idx + 1] = dp[idx];
            if prev_char == b'2' {
                // "27" is not valid so "7" needs to make its own letter.
                // In contrast, "25" can make a letter.
                if curr_char <= b'6' {
                    dp[idx + 1] += dp[idx - 1];
                }
            } else if prev_char == b'1' {
                // "11" through "19" can all make valid letters.
                dp[idx + 1] += dp[idx - 1];
            }
        }

        return dp[dp.len() - 1];
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
