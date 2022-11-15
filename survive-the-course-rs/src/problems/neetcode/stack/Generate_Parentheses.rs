// https://leetcode.com/problems/generate-parentheses/

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut curr_stack = Vec::new();
        Self::gen_parens_recursive(&mut curr_stack, 0, n, &mut result);

        return result;
    }

    fn gen_parens_recursive(
        stack: &mut Vec<char>,
        opens: i32,
        remaining: i32,
        result: &mut Vec<String>,
    ) {
        if opens == 0 && remaining == 0 {
            result.push(stack.iter().collect());
            return;
        }

        if remaining > 0 {
            stack.push('(');
            Self::gen_parens_recursive(stack, opens + 1, remaining - 1, result);
            stack.pop();
        }

        if opens > 0 {
            stack.push(')');
            Self::gen_parens_recursive(stack, opens - 1, remaining, result);
            stack.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util::assert_eq_ignore_order;

    #[test]
    fn test_1() {
        assert_eq_ignore_order(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
    }
}
