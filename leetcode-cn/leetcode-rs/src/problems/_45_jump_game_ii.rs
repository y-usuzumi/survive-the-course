pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut next_farthest = 0;
        let mut current_farthest = 0;
        let mut steps = 0;
        for idx in 0..nums.len() {
            if idx > current_farthest {
                current_farthest = next_farthest;
                steps += 1;
            }
            next_farthest = std::cmp::max(next_farthest, idx + nums[idx] as usize);
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
    }
}