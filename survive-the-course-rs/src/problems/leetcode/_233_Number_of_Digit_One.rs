// https://leetcode.com/problems/number-of-digit-one/

pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n >= 1 && n < 10 {
            return 1;
        }

        let log10 = (n as f64).log10() as u32;
        let base = 10_i32.pow(log10);
        let remainder = n % base;
        let mut result = 0;
        let remainder_digits = Self::count_digit_one(remainder);
        if n >= 2 * base {
            result += (n / base + 1) * Self::count_digit_one_by_log10(log10 - 1);
            result += base;
        } else {
            result += n - base;
            result += remainder_digits;
        }

        result += remainder_digits;
        return result;
    }

    pub fn count_digit_one_by_log10(n: u32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut n0 = n;
        let mut result = 10_i32.pow(n0);
        while n0 > 0 {
            result += 10_i32.pow(n as u32 - 1);
            n0 -= 1;
        }
        return result + Self::count_digit_one_by_log10(n - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digit_one_by_log10_1() {
        assert_eq!(Solution::count_digit_one_by_log10(2), 132);
    }

    #[test]
    fn test_count_digit_one_by_log10_2() {
        assert_eq!(Solution::count_digit_one(33), 15);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_digit_one(13), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_digit_one(0), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_digit_one(133), 58);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::count_digit_one(21), 13);
    }
}
