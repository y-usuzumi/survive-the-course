// https://leetcode.com/problems/substring-with-largest-variance/

pub struct Solution;

impl Solution {
    // Iterate over all possibilities of two-char combinations. Consider one
    // char (a) as the one with the most occurrences, and the other (b) with the
    // fewest occurrences. If we assign 1 to char a and -1 to char b, then the
    // problem is converted to a variant of the "maximum subarray" problem
    // (https://leetcode.com/problems/maximum-subarray/description/)
    //
    // However, this variant requires DP. For each position i in the string,
    // Consider: f[i] to be the max sum of subarray up to and include the i
    // position (but can be empty so the sum is 0). g[i] to be the max sum of subarray
    // up to and include the i position **with at least one -1 (char b). Let
    // f[i] = 0 and g[i] = -infinity.
    // Recurrence relation:
    // when s[i] is char a:
    //   f[i] = f[i-1] + 1
    //   g[i] = g[i-1] + 1
    // when s[i] is char b:
    //   f[i] = max(f[i-1] - 1, 0)
    //   g[i] = f[i-1] - 1
    // else: skip.
    // Then for this combination of char a and char b, the variance is max(g[..])
    pub fn largest_variance(s: String) -> i32 {
        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();
        for ref ch1 in 'a'..='z' {
            for ref ch2 in 'a'..='z' {
                if ch1 == ch2 {
                    continue;
                }

                let mut max_sum = 0;
                let mut max_sum_with_neg = -100000;
                for ch in &chars {
                    if ch == ch1 {
                        max_sum += 1;
                        max_sum_with_neg += 1;
                    } else if ch == ch2 {
                        max_sum_with_neg = max_sum - 1;
                        max_sum = (max_sum - 1).max(0);
                    }
                }
                result = result.max(max_sum_with_neg);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
