package survive.the.course.java.leetcode._947_Most_Stones_Removed_with_Same_Row_or_Column;

import java.util.Arrays;

// Disjoint set
// If two stones are in the same row or column, the are in the same set.
// Disjoint set is built with a one-dimensional array with the size of
// `row_count` + `col_count` of the board.
public class Solution {
    private void connect(int[] ds, int rowIdx, int colIdx) {
        if (ds[rowIdx] == -1) {
            ds[rowIdx] = rowIdx;
        }

        if (ds[colIdx] == -1) {
            ds[colIdx] = rowIdx;
        }
        int rowRoot = rowIdx;
        while (rowRoot != ds[rowRoot]) {
            rowRoot = ds[rowRoot];
        }

        int colRoot = colIdx;
        while (colRoot != ds[colRoot]) {
            colRoot = ds[colRoot];
        }

        ds[colRoot] = rowRoot;
    }

    public int removeStones(int[][] stones) {
        if (stones.length == 0 || stones[0].length == 0) {
            return 0;
        }

        int rows = 0;
        int cols = 0;
        for (int idx = 0; idx < stones.length; idx++) {
            rows = Math.max(rows, stones[idx][0] + 1);
            cols = Math.max(cols, stones[idx][1] + 1);
        }

        int[] ds = new int[rows + cols];
        Arrays.fill(ds, -1);

        for (int idx = 0; idx < stones.length; idx++) {
            connect(ds, stones[idx][0], stones[idx][1] + rows);
        }

        int result = 0;
        for (int idx = 0; idx < rows; idx++) {
            if (ds[idx] == idx) {
                result += 1;
            }
        }

        return stones.length - result;
    }
}
