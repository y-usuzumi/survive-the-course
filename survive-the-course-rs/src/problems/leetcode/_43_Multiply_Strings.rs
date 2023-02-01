// https://leetcode.com/problems/multiply-strings/

pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        fn merge(n1: &mut Vec<u8>, n2: Vec<u8>) {
            let mut carry = 0;
            let mut idx = 0;
            while idx < n1.len() || idx < n2.len() {
                let &n2digit = n2.get(idx).unwrap_or(&0);
                if let Some(n1digit) = n1.get_mut(idx) {
                    let digit = n2digit + *n1digit + carry;
                    *n1digit = digit % 10;
                    carry = digit / 10;
                } else {
                    let digit = n2digit + carry;
                    n1.push(digit % 10);
                    carry = digit / 10;
                }
                idx += 1;
            }
            if carry > 0 {
                n1.push(carry);
            }
        }
        let num1digits: Vec<u8> = num1.chars().rev().map(|ch| ch as u8 - '0' as u8).collect();
        let num2digits: Vec<u8> = num2.chars().rev().map(|ch| ch as u8 - '0' as u8).collect();
        let mut result_chars: Vec<u8> = vec![];
        let mut zeros = 0;
        for &num2digit in num2digits.iter() {
            let mut single_digit_result: Vec<u8> = std::iter::repeat(0).take(zeros).collect();
            let mut carry = 0;
            for &num1digit in num1digits.iter() {
                let num12 = num1digit * num2digit + carry;
                carry = num12 / 10;
                single_digit_result.push(num12 % 10);
            }
            if carry > 0 {
                single_digit_result.push(carry);
            }
            merge(&mut result_chars, single_digit_result);

            zeros += 1;
        }

        return String::from_iter(
            result_chars
                .into_iter()
                .rev()
                .map(|digit| (digit + '0' as u8) as char),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
