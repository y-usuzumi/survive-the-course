pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            mut n => {
                let (mut prev, mut curr) = (1, 2);
                while n > 2 {
                    let next = prev + curr;
                    prev = curr;
                    curr = next;
                    n -= 1;
                }
                curr
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}