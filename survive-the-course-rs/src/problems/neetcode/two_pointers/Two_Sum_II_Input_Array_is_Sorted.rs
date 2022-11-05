// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idxl = 0;
        let mut idxr = numbers.len() - 1;

        while idxl < idxr {
            match (numbers[idxl] + numbers[idxr]).cmp(&target) {
                std::cmp::Ordering::Equal => {
                    return vec![idxl as i32 + 1, idxr as i32 + 1];
                }
                std::cmp::Ordering::Less => {
                    idxl += 1;
                }
                std::cmp::Ordering::Greater => {
                    idxr -= 1;
                }
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
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
