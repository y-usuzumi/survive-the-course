// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    // This solution uses what is called 0-1 BFS which closely resembles
    // Dijkstra's algorithm. It requires the edges to have a weight of either 0
    // or 1. In Dijkstra's algorithm, since the weight can be any non-negative
    // values, we need a PriorityQueue to ensure we always pop the node with the
    // shortest path. In 0-1 BFS, we can simply use a double-ended list. If the
    // weight of the edge is 0, we prepend the next node to the list, otherwise
    // we append it to the last.
    //
    // In this problem, finding minimum obstacles to remove is equivalent to
    // finding the minimum cost of path.
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut costs = vec![vec![i32::MAX; cols]; rows];
        costs[0][0] = grid[0][0];
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while let Some((row, col)) = q.pop_front() {
            if visited[row][col] {
                continue;
            }
            if row == rows - 1 && col == cols - 1 {
                return costs[row][col];
            }
            visited[row][col] = true;
            let cost = costs[row][col];
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = row as i32 + dx;
                let y = col as i32 + dy;
                if x < 0 || x >= rows as i32 || y < 0 || y >= cols as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                let dist = grid[x][y];
                if cost + dist < costs[x][y] {
                    costs[x][y] = cost + dist;
                    if dist == 1 {
                        q.push_back((x, y));
                    } else {
                        q.push_front((x, y));
                    }
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
        todo!();
    }
}
