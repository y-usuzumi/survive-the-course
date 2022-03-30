struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(temperatures.len() / 4);
        let mut result = vec![0; temperatures.len()];
        for (idx, temp) in temperatures.into_iter().enumerate() {
            while !stack.is_empty() {
                let (last_idx, last_temp) = stack.last().unwrap();
                if *last_temp < temp {
                    result[*last_idx] = (idx - last_idx) as i32;
                    stack.pop();
                    continue;
                }
                break;
            }
            stack.push((idx, temp));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]), vec![1,1,4,2,1,1,0,0]);
        assert_eq!(Solution::daily_temperatures(vec![30,40,50,60]), vec![1,1,1,0]);
        assert_eq!(Solution::daily_temperatures(vec![30,60,90]), vec![1,1,0]);
        assert_eq!(Solution::daily_temperatures(vec![30]), vec![0]);
    }
}