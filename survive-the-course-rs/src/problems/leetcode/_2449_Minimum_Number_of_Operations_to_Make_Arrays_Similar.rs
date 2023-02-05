// https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar/

pub struct Solution;

impl Solution {
    pub fn make_similar(mut nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        nums.sort_unstable();
        target.sort_unstable();
        let mut nums_even = vec![];
        let mut nums_odd = vec![];
        for num in nums {
            if num % 2 == 0 {
                nums_even.push(num);
            } else {
                nums_odd.push(num);
            }
        }

        let mut target_even = vec![];
        let mut target_odd = vec![];
        for num in target {
            if num % 2 == 0 {
                target_even.push(num);
            } else {
                target_odd.push(num);
            }
        }

        let mut result = 0i64;
        for idx in 0..nums_even.len() {
            if nums_even[idx] < target_even[idx] {
                result += ((target_even[idx] - nums_even[idx]) / 2) as i64;
            }
        }
        for idx in 0..nums_odd.len() {
            if nums_odd[idx] < target_odd[idx] {
                result += ((target_odd[idx] - nums_odd[idx]) / 2) as i64;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
