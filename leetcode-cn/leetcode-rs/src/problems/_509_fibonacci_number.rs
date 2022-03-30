struct Solution;

impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        let (mut prev, mut curr) = (0, 1);
        if n == 0 {
            return prev;
        }
        if n == 1 {
            return curr;
        }
        while n > 1 {
            (prev, curr) = (curr, prev + curr);
            n -= 1;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}