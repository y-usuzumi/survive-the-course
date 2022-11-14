package survive.the.course.java.leetcode._130_Surrounded_Regions;

public class Solution {
    private static final int[][] directions = new int[][]{{-1, 0}, {0, 1}, {1, 0}, {0, -1}};

    public void solve(char[][] board) {
        int rows = board.length;
        int cols = board[0].length;

        if (rows < 3 || cols < 3) {
            return;
        }

        for (int colIdx = 0; colIdx < cols; colIdx++) {
            if (board[0][colIdx] == 'O') {
                bfs(board, 0, colIdx);
            }
            if (board[rows-1][colIdx] == 'O') {
                bfs(board, rows-1, colIdx);
            }
        }

        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            if (board[rowIdx][0] == 'O') {
                bfs(board, rowIdx, 0);
            }
            if (board[rowIdx][cols-1] == 'O') {
                bfs(board, rowIdx, cols-1);
            }
        }

        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            for (int colIdx = 0; colIdx < cols; colIdx++) {
                if (board[rowIdx][colIdx] == 'O') {
                    board[rowIdx][colIdx] = 'X';
                } else if (board[rowIdx][colIdx] == 'o') {
                    board[rowIdx][colIdx] = 'O';
                }
            }
        }
    }

    private void bfs(char[][] board, int rowIdx, int colIdx) {
        if (rowIdx < 0 || rowIdx >= board.length || colIdx < 0 || colIdx >= board[0].length) {
            return;
        }
        if (board[rowIdx][colIdx] != 'O') {
            return;
        }
        board[rowIdx][colIdx] = 'o';
        for (int[] direction : directions) {
            bfs(board, rowIdx + direction[0], colIdx + direction[1]);
        }
    }
}
