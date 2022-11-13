package survive.the.course.java.leetcode._200_Number_of_Islands;

import java.util.Arrays;

/**
 * This solution utilizes Disjoint Sets. The idea is to create a disjoint set from each location,
 * and perform merge with its left and upper locations. At the end we count the number of root
 * nodes of each set.
 */
class Solution {
    public int numIslands(char[][] grid) {
        if (grid.length == 0 || grid[0].length == 0) {
            return 0;
        }
        int rows = grid.length;
        int cols = grid[0].length;
        int[][] ds = new int[rows][];
        for (int i = 0; i < rows; i++) {
            ds[i] = new int[cols];
            Arrays.fill(ds[i], -2);
        }

        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            for (int colIdx = 0; colIdx < cols; colIdx++) {
                if (grid[rowIdx][colIdx] == '1') { // land
                    ds[rowIdx][colIdx] = -1;
                    if (colIdx > 0 && grid[rowIdx][colIdx - 1] == '1') {
                        findRootAndMerge(ds, cols, rowIdx, colIdx, rowIdx, colIdx - 1);
                    }
                    if (rowIdx > 0 && grid[rowIdx - 1][colIdx] == '1') {
                        findRootAndMerge(ds, cols, rowIdx, colIdx, rowIdx - 1, colIdx);
                    }
                }
            }
        }
        int result = 0;
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (ds[i][j] == -1) {
                    result++;
                }
            }
        }
        return result;
    }

    private boolean findRootAndMerge(int[][] ds, int cols, int rowIdx1, int colIdx1, int rowIdx2, int colIdx2) {
        int root1 = ds[rowIdx1][colIdx1];
        while (root1 != -1) {
            rowIdx1 = root1 / cols;
            colIdx1 = root1 % cols;
            root1 = ds[rowIdx1][colIdx1];
        }

        int root2 = ds[rowIdx2][colIdx2];
        while (root2 != -1) {
            rowIdx2 = root2 / cols;
            colIdx2 = root2 % cols;
            root2 = ds[rowIdx2][colIdx2];
        }
        if (rowIdx1 == rowIdx2 && colIdx1 == colIdx2) {
            return false;
        } else {
            ds[rowIdx1][colIdx1] = rowIdx2 * cols + colIdx2;
            return true;
        }
    }
}