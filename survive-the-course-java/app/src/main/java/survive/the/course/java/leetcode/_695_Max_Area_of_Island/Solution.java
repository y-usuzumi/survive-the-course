package survive.the.course.java.leetcode._695_Max_Area_of_Island;

public class Solution {
    public int maxAreaOfIsland(int[][] grid) {
        if (grid.length == 0 || grid[0].length == 0) {
            return 0;
        }

        int rows = grid.length;
        int cols = grid[0].length;
        Integer[][] ds = new Integer[rows][];
        for (int rowIdx = 0; rowIdx < grid.length; rowIdx++) {
            ds[rowIdx] = new Integer[cols];
        }

        int result = 0;

        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            for (int colIdx = 0; colIdx < cols; colIdx++) {
                if (grid[rowIdx][colIdx] == 0) {
                    continue;
                }
                ds[rowIdx][colIdx] = -1;
                if (result == 0) {
                    result = 1;
                }
                if (colIdx > 0 && grid[rowIdx][colIdx-1] > 0) {
                    int newSize = unionAndGetSize(ds, cols, rowIdx, colIdx-1, rowIdx, colIdx);
                    if (newSize > result) {
                        result = newSize;
                    }
                }
                if (rowIdx > 0 && grid[rowIdx-1][colIdx] > 0) {
                    int newSize = unionAndGetSize(ds, cols, rowIdx-1, colIdx, rowIdx, colIdx);
                    if (newSize > result) {
                        result = newSize;
                    }
                }
            }
        }
        return result;
    }

    private int unionAndGetSize(Integer[][] ds, int cols, int rowIdx1, int colIdx1, int rowIdx2, int colIdx2) {
        Integer root1 = ds[rowIdx1][colIdx1];
        while (root1 >= 0) {
            rowIdx1 = root1 / cols;
            colIdx1 = root1 % cols;
            root1 = ds[rowIdx1][colIdx1];
        }

        Integer root2 = ds[rowIdx2][colIdx2];
        while (root2 >= 0) {
            rowIdx2 = root2 / cols;
            colIdx2 = root2 % cols;
            root2 = ds[rowIdx2][colIdx2];
        }

        if (rowIdx1 == rowIdx2 && colIdx1 == colIdx2) {
            return -root1;
        }

        if (root1 > root2) {
            Integer tmp = root1;
            root1 = root2;
            root2 = tmp;
        }
        ds[rowIdx2][colIdx2] = rowIdx1 * cols + colIdx1;
        ds[rowIdx1][colIdx1] = root1 + root2;
        return -(root1 + root2);
    }
}
