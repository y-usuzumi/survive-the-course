// https://leetcode.com/problems/combination-sum/

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        pub fn helper(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
            if target == 0 {
                return vec![vec![]];
            }

            if target < 0 {
                return vec![];
            }

            let mut results = vec![];
            for idx in 0..candidates.len() {
                let curr_results = helper(&candidates[idx..], target - candidates[idx]);
                for mut result in curr_results {
                    result.push(candidates[idx]);
                    results.push(result);
                }
            }

            return results;
        }

        return helper(&candidates, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
