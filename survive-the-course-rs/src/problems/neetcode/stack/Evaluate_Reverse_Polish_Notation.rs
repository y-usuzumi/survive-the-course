// https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operands: Vec<i32> = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();
                    operands.push(left + right);
                }
                "-" => {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();
                    operands.push(left - right);
                }
                "*" => {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();
                    operands.push(left * right);
                }
                "/" => {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();
                    operands.push(left / right);
                }
                num => {
                    operands.push(num.parse().unwrap());
                }
            }
        }

        return *operands.last().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let rpn = ["2", "1", "+", "3", "*"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::eval_rpn(rpn), 9);
    }
}
