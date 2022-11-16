// https://leetcode.com/problems/network-delay-time/

use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    // Dijkstra
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut edge_map = HashMap::new();
        for time in times {
            let edges: &mut Vec<(i32, i32)> = match edge_map.entry(time[0]) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(Vec::new()),
            };
            edges.push((time[1], time[2]));
        }

        let mut visited = vec![false; (n + 1) as usize];
        let mut visited_count = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, k)));

        while !heap.is_empty() {
            let Reverse((dist, node)) = heap.pop().unwrap();
            if visited[node as usize] {
                continue;
            }
            if visited_count == n - 1 {
                return dist;
            }
            let edges = edge_map.get(&node);
            if let Some(edges) = edges {
                for (target, weight) in edges {
                    if visited[*target as usize] {
                        continue;
                    }
                    heap.push(Reverse((dist + weight, *target)));
                }
            }
            visited[node as usize] = true;
            visited_count += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
    }
}
