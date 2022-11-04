// https://leetcode.com/problems/valid-sudoku/

pub struct Solution;

impl Solution {
    // 用set或数组保存出现占用过的行、列及3x3方格
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_lookup = vec![0; 9];
        let mut col_lookup = vec![0; 9];
        let mut block_lookup = vec![0; 9];
        for (row_idx, row) in board.into_iter().enumerate() {
            for (col_idx, ch) in row.into_iter().enumerate() {
                if ch == '.' {
                    continue;
                }
                let num = ch as u8 - '0' as u8;
                if row_lookup[row_idx] & (1 << num) > 0 || col_lookup[col_idx] & (1 << num) > 0 {
                    return false;
                }
                let block_idx = row_idx / 3 * 3 + col_idx / 3;
                if block_lookup[block_idx] & (1 << num) > 0 {
                    return false;
                }
                row_lookup[row_idx] |= 1 << num;
                col_lookup[col_idx] |= 1 << num;
                block_lookup[block_idx] |= 1 << num;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn test_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
