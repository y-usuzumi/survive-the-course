use std::collections::HashMap;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for num in nums.into_iter() {
            m.insert(num, m.get(&num).unwrap_or(&0) + 1);
        }
        let mut all_keys: Vec<i32> = m.keys().copied().collect();
        all_keys.sort();

        let mut result: HashMap<i32, i32> = HashMap::new();
        let (mut last_num_2, mut last_num_1) = (0, 0);
        for idx in 0..all_keys.len() {
            let num = all_keys[idx];
            if last_num_1 == 0 {
                result.insert(num, m[&num] * num);
                last_num_1 = num;
            } else if num - last_num_1 > 1 {
                result.insert(num, result[&last_num_1] + m[&num] * num);
                last_num_2 = last_num_1;
                last_num_1 = num;
            } else if last_num_2 == 0 {
                result.insert(num, max(result[&last_num_1], m[&num] * num));
                last_num_2 = last_num_1;
                last_num_1 = num;
            } else {
                result.insert(num, max(result[&last_num_2] + m[&num] * num, result[&last_num_1]));
                last_num_2 = last_num_1;
                last_num_1 = num;
            }
        }
        if result.len() == 1 {
            result[&last_num_1]
        } else {
            max(result[&last_num_2], result[&last_num_1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::delete_and_earn(vec![1,1,1,2,4,5,5,5,6]), 18);
    }
}
