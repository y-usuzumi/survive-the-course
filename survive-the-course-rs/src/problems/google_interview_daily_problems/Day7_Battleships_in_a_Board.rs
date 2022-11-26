// https://leetcode.com/problems/battleships-in-a-board/

pub trait Solution {
    fn count_battleships(board: Vec<Vec<char>>) -> i32;
}

pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    fn dfs(row_idx: usize, col_idx: usize, board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
        if board[row_idx][col_idx] != 'X' || visited[row_idx][col_idx] {
            return;
        }
        visited[row_idx][col_idx] = true;
        if row_idx < board.len() - 1 {
            Self::dfs(row_idx + 1, col_idx, board, visited);
        }
        if col_idx < board[0].len() - 1 {
            Self::dfs(row_idx, col_idx + 1, board, visited);
        }
        if row_idx > 0 {
            Self::dfs(row_idx - 1, col_idx, board, visited);
        }
        if col_idx > 0 {
            Self::dfs(row_idx, col_idx - 1, board, visited);
        }
    }
}

impl Solution for Solution1 {
    // Regular DFS
    fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.len() == 0 || board[0].len() == 0 {
            return 0;
        }
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut result = 0;

        for row_idx in 0..board.len() {
            for col_idx in 0..board[0].len() {
                if board[row_idx][col_idx] == 'X' && !visited[row_idx][col_idx] {
                    Self::dfs(row_idx, col_idx, &board, &mut visited);
                    result += 1;
                }
            }
        }

        return result;
    }
}

impl Solution for Solution2 {
    // The topmost or leftmost 'X' of a ship marks the ship. All adjacent 'X's
    // row/column-wise are ignored.
    fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.len() == 0 || board[0].len() == 0 {
            return 0;
        }

        let mut result = 0;

        for row_idx in 0..board.len() {
            for col_idx in 0..board[0].len() {
                if board[row_idx][col_idx] == 'X' {
                    if (row_idx == 0 || row_idx > 0 && board[row_idx - 1][col_idx] != 'X')
                        && (col_idx == 0 || col_idx > 0 && board[row_idx][col_idx - 1] != 'X')
                    {
                        result += 1;
                    }
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::function(), 0);
    }
}
