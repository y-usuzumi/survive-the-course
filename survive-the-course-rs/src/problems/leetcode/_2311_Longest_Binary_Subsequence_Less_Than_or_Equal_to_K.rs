// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/

pub trait Solution {
    fn longest_subsequence(s: String, k: i32) -> i32;
}

pub struct Solution1;
pub struct Solution2;

impl Solution for Solution1 {
    // Stupid
    fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut counter = 0;
        let mut num = 0;
        for ch in s.chars() {
            num <<= 1;
            counter += 1;
            match ch {
                '0' => {}
                '1' => {
                    num |= 1;
                }
                _ => panic!("Impossible"),
            }
            if num > k {
                let n = !(1_i32.rotate_right(num.leading_zeros() + 1));
                num &= n;
                counter -= 1;
            }
        }
        return counter;
    }
}

impl Solution for Solution2 {
    // Two principles:
    // 1. We keep all zeros
    // 2. We do not sacrifice 0's for 1's.
    // The reason why the above principles hold is:
    // 1. Consider the subsequence of all zeros. Obviously it <= k.
    // 2. If adding a 1 requires shifting 0's out, keeping 0's are apparently
    //    more economical because it is smaller.
    fn longest_subsequence(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut sum = 0;
        let mut bits = 0;
        let mut seq_len = 0;
        let mut skip_all_ones = false;
        for idx in (0..chars.len()).rev() {
            if chars[idx] == '1' {
                if skip_all_ones {
                    continue;
                }
                // if bits > 30, it is the sign bit. We should ignore it.
                if bits > 30 || sum + (1 << bits) > k {
                    skip_all_ones = true;
                } else {
                    sum += 1 << bits;
                    seq_len += 1;
                }
            } else {
                seq_len += 1;
            }
            bits += 1;
        }

        return seq_len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution1 {
        use super::*;
        #[test]
        fn test_1() {
            assert_eq!(Solution1::longest_subsequence("1001010".into(), 5), 5);
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution1::longest_subsequence("00101001".into(), 1), 6);
        }
    }

    mod solution2 {
        use super::*;
        #[test]
        fn test_1() {
            assert_eq!(Solution2::longest_subsequence("1001010".into(), 5), 5);
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution2::longest_subsequence("00101001".into(), 1), 6);
        }
    }
}
