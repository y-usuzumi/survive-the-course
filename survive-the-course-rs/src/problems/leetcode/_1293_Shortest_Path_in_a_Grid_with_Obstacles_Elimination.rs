// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/description/

pub struct Solution;

impl Solution {
    // BFS traversal. Eliminating k obstacles is equivalent to crossing k
    // obstacles to reach the goal. We maintain a map between the coordinates
    // and current minimal cost Starting from the top-left corner, which has a
    // cost of 0 (or maybe 1 if the starting point itself has an obstacle on
    // it). Each time we take one step and reach the next possible blocks. If
    // the block has an obstacle, we increase the cost and put the (coord, cost)
    // into the queue. Apparently if cost is already greater than the given k
    // it means we have run out of k and this path is invalid.
    // Whenever we take a (coord, cost) from the queue, if the cost is less than
    // the previous cost at the same position, it means we have found an alternative
    // path to the position with more steps and less cost. Since it takes more
    // steps, it will come later in the queue, so it will only be considered
    // if the shorter path with more cost does not satisfy the k constraint.
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut q = Vec::new();
        let mut costs = vec![vec![i32::MAX; cols]; rows];
        q.push((0, 0, grid[0][0]));
        let mut step = 0;
        while !q.is_empty() {
            let mut next_q = Vec::new();
            for (row, col, cost) in q {
                if cost > k {
                    continue;
                }
                if cost > costs[row][col] {
                    // Current path takes more steps and cost even more. Drop
                    // it.
                    continue;
                }
                if row == rows - 1 && col == cols - 1 {
                    return step;
                }
                costs[row][col] = cost;
                if row < rows - 1 {
                    next_q.push((row + 1, col, cost + grid[row + 1][col]));
                }
                if col < cols - 1 {
                    next_q.push((row, col + 1, cost + grid[row][col + 1]));
                }
                if row > 0 {
                    next_q.push((row - 1, col, cost + grid[row - 1][col]));
                }
                if col > 0 {
                    next_q.push((row, col - 1, cost + grid[row][col - 1]));
                }
            }
            q = next_q;
            step += 1;
        }
        return -1;
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
