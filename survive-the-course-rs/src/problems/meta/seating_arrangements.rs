// <Place the leetcode link to the question here>

pub struct Solution;

impl Solution {
    pub fn min_overall_awkwardness(arr: &mut Vec<i32>) -> i32 {
        arr.sort();

        if arr.len() < 2 {
            return 0;
        }

        let mut min_overall_awkwardness = 0;
        for idx in (0..(arr.len() - 2)).step_by(2) {
            min_overall_awkwardness = std::cmp::max(min_overall_awkwardness, (arr[idx] - arr[idx+2]).abs());
        }

        for idx in (1..(arr.len() - 2)).step_by(2) {
            min_overall_awkwardness = std::cmp::max(min_overall_awkwardness, (arr[idx] - arr[idx+2]).abs());
        }

        min_overall_awkwardness = std::cmp::max(min_overall_awkwardness, (arr[0] - arr[1]).abs());

        min_overall_awkwardness = std::cmp::max(min_overall_awkwardness, (arr[arr.len() - 1] - arr[arr.len() - 2]).abs());

        min_overall_awkwardness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_overall_awkwardness(&mut vec![5, 10, 6, 8]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_overall_awkwardness(&mut vec![1, 2, 5, 3, 7]), 4);
    }
}