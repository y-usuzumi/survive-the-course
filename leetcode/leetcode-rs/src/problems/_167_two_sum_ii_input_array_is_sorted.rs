struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left_idx, mut right_idx) = (0, numbers.len() - 1);
        while left_idx < right_idx {
            let sum = numbers[left_idx] + numbers[right_idx];
            if sum == target {
                return vec![(left_idx + 1) as i32, (right_idx + 1) as i32];
            }
            if sum < target {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }
        panic!("Impossible");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}