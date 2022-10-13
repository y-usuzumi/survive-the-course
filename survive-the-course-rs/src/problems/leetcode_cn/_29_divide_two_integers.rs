// https://leetcode-cn.com/problems/divide-two-integers/

pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }

        if divisor == -1 {
            return -dividend;
        }

        if dividend > 0 && divisor < 0 || dividend < 0 && divisor > 0 {
            return -(dividend / divisor);
        }

        let result = 0;
        let mut divisor_shifted = divisor;
        let mut bit_shifted = 1;
        while dividend > divisor_shifted && dividend > 0 {
            divisor_shifted <<= 1;
            bit_shifted <<= 1;
        }

        unimplemented!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        unimplemented!("TODO");
    }
}
