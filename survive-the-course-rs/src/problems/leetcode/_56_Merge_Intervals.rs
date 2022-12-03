// https://leetcode.com/problems/merge-intervals/

pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        intervals.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut left = intervals[0][0];
        let mut right = intervals[0][1];
        for idx in 1..intervals.len() {
            let interval = &intervals[idx];
            if interval[0] > right {
                result.push(vec![left, right]);
                left = interval[0];
                right = interval[1];
            } else {
                right = right.max(interval[1]);
            }
        }

        result.push(vec![left, right]);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}
