// https://leetcode.com/problems/remove-k-digits/

pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack = vec![];
        for digit in num.chars() {
            let digit = digit as u8 - '0' as u8;
            if k == 0 || stack.is_empty() || digit >= *stack.last().unwrap() {
                stack.push(digit);
            } else {
                while let Some(&elem) = stack.last() {
                    if elem > digit && k > 0 {
                        stack.pop();
                        k -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(digit);
            }
        }

        for _ in 0..k {
            stack.pop();
        }

        let mut result: String = stack
            .into_iter()
            .skip_while(|&v| v == 0)
            .map(|v| ('0' as u8 + v) as char)
            .collect();
        if result.is_empty() {
            result.push('0');
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219");
    }
}
