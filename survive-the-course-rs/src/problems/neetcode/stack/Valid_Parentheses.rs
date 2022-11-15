// https://leetcode.com/problems/valid-parentheses/

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => {
                    stack.push(ch);
                }
                ')' => {
                    if stack.len() > 0 && *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if stack.len() > 0 && *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if stack.len() > 0 && *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => panic!("Impossible!"),
            }
        }

        return stack.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }
}
