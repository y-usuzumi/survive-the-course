// https://leetcode.com/problems/largest-rectangle-in-histogram/

pub struct Solution;

impl Solution {
    // We use a stack. When we iterate over the heights, if the
    // current height is greater than the top element in the stack, we push it
    // into the stack. The idea is when we have a height smaller than the top
    // element in the stack, the area in effect is bounded to right before the
    // current position.
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights_len = heights.len();
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut curr_max = 0;
        for (idx, height) in heights.into_iter().enumerate() {
            let mut leftmost_idx = idx;
            while !stack.is_empty() && stack.last().unwrap().1 > height {
                let (prev_idx, prev_height) = stack.pop().unwrap();
                curr_max = curr_max.max((idx - prev_idx) as i32 * prev_height);
                leftmost_idx = prev_idx;
            }
            stack.push((leftmost_idx, height));
        }

        while !stack.is_empty() {
            let (idx, height) = stack.pop().unwrap();
            curr_max = curr_max.max((heights_len - idx) as i32 * height);
        }

        return curr_max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
