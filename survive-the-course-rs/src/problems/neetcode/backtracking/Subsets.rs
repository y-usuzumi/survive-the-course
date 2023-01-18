// https://leetcode.com/problems/subsets/

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        for num in nums {
            for idx in 0..result.len() {
                let mut v = result[idx].clone();
                v.push(num);
                result.push(v);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
