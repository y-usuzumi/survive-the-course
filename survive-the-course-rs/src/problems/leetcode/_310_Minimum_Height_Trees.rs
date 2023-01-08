// https://leetcode.com/problems/minimum-height-trees/

use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    // Topological sort until we have one or two nodes.
    // For better explanation, check the link to the problem.
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut hm = HashMap::new();
        let mut prereqs = vec![0 as usize; n as usize];
        for idx in 0..n as usize {
            hm.insert(idx, Vec::new());
        }
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            hm.get_mut(&from).unwrap().push(to);
            hm.get_mut(&to).unwrap().push(from);
            prereqs[from] += 1;
            prereqs[to] += 1;
        }

        let mut q = VecDeque::new();
        for (node, prereq) in prereqs.iter().enumerate() {
            if *prereq == 1 {
                q.push_back(node);
            }
        }

        let mut remaining_nodes = n;

        while remaining_nodes > 2 && !q.is_empty() {
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                remaining_nodes -= 1;
                let children = hm.get(&node).unwrap();
                for child in children {
                    if prereqs[*child] == 0 {
                        continue;
                    }
                    prereqs[*child] -= 1;
                    if prereqs[*child] == 1 {
                        q.push_back(*child);
                    }
                }
            }
        }

        return q.iter().map(|v| *v as i32).collect();
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
