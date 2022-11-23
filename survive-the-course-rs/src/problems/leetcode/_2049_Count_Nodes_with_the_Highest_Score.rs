// https://leetcode.com/problems/count-nodes-with-the-highest-score/

use std::collections::{hash_map::Entry, HashMap, HashSet};

pub struct Solution;

impl Solution {
    fn dfs(
        curr_node: usize,
        total_nodes: i32,
        edge_map: &[Vec<usize>],
        sizes: &mut [i32],
    ) -> (i128, i32) {
        let mut size = 1;
        let mut max_score = 0;
        let mut max_count = 0;
        let mut my_score: i128 = 1;
        for edge in &edge_map[curr_node] {
            let (curr_max_score, curr_max_count) = Self::dfs(*edge, total_nodes, edge_map, sizes);
            if curr_max_score > max_score {
                max_score = curr_max_score;
                max_count = curr_max_count;
            } else if curr_max_score == max_score {
                max_count += curr_max_count;
            }
            my_score *= sizes[*edge] as i128;
            size += sizes[*edge];
        }
        sizes[curr_node as usize] = size;
        my_score *= (total_nodes - size).max(1) as i128;
        if my_score > max_score {
            max_score = my_score;
            max_count = 1;
        } else if my_score == max_score {
            max_count += 1;
        }
        return (max_score, max_count);
    }

    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut edge_map = vec![Vec::new(); parents.len()];
        let mut sizes: Vec<i32> = vec![0; parents.len()];

        for idx in 1..parents.len() {
            edge_map[parents[idx] as usize].push(idx);
        }

        let (_max_score, max_count) = Self::dfs(0, parents.len() as i32, &edge_map, &mut sizes);

        return max_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0]), 2);
    }

    #[test]
    fn test_3() {
        let test_data: Vec<i32> =
            serde_json::from_slice(include_bytes!("resources/_2049_more_test_cases_1.txt"))
                .unwrap();
        assert_eq!(Solution::count_highest_score_nodes(test_data), 1);
    }

    #[test]
    fn test_4() {
        let test_data: Vec<i32> =
            serde_json::from_slice(include_bytes!("resources/_2049_more_test_cases_2.txt"))
                .unwrap();
        assert_eq!(Solution::count_highest_score_nodes(test_data), 1);
    }
}
