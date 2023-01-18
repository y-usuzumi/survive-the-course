// https://leetcode.com/problems/permutations/

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32]) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                return vec![vec![]];
            }

            let subresults = helper(&nums[1..]);
            let mut results = vec![];
            for subresult in subresults {
                for idx in 0..=subresult.len() {
                    let result = [&subresult[0..idx], &[nums[0]], &subresult[idx..]].concat();
                    results.push(result);
                }
            }
            return results;
        }

        return helper(&nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
