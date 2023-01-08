// https://leetcode.com/problems/find-eventual-safe-states/

use std::collections::{HashMap, VecDeque};

pub struct Solution1;
// This problem can also be solved by Top-down DP with memoization
pub struct Solution2;

impl Solution1 {
    // Reversed topological sort
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let len = graph.len();
        let mut hm = HashMap::new();
        for node in 0..len as i32 {
            hm.insert(node, vec![]);
        }

        let mut prereqs = vec![0; len];

        for (idx, children) in graph.iter().enumerate() {
            prereqs[idx] = children.len();
            for child in children {
                hm.get_mut(child).unwrap().push(idx as i32);
            }
        }

        let mut q = VecDeque::new();
        for (idx, prereq) in prereqs.iter().enumerate() {
            if *prereq == 0 {
                q.push_back(idx as i32);
            }
        }

        let mut result = vec![];

        while let Some(node) = q.pop_front() {
            result.push(node);
            for &child in hm.get(&node).unwrap() {
                prereqs[child as usize] -= 1;
                if prereqs[child as usize] == 0 {
                    q.push_back(child);
                }
            }
        }

        result.sort();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
