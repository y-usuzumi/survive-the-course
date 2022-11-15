// https://leetcode.com/problems/daily-temperatures/

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];

        let mut temperature_stack = Vec::new();
        let mut idx_stack = Vec::new();
        for (idx, temperature) in temperatures.into_iter().enumerate() {
            while !temperature_stack.is_empty() && *temperature_stack.last().unwrap() < temperature
            {
                temperature_stack.pop().unwrap();
                let last_idx = idx_stack.pop().unwrap();
                result[last_idx] = (idx - last_idx) as i32;
            }
            temperature_stack.push(temperature);
            idx_stack.push(idx);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
