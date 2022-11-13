package survive.the.course.java.leetcode._417_Pacific_Atlantic_Water_Flow;

import java.util.ArrayList;
import java.util.List;

class Solution {
    private static final int[][] directions = new int[][] { { -1, 0 }, { 0, 1 }, { 1, 0 }, { 0, -1 } };

    public List<List<Integer>> pacificAtlantic(int[][] heights) {
        int rows = heights.length;
        int cols = heights[0].length;
        int[][] flows = new int[rows][];
        List<List<Integer>> result = new ArrayList<>();
        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            flows[rowIdx] = new int[cols];
        }

        for (int colIdx = 0; colIdx < cols; colIdx++) {
            bfs(heights, flows, 0, colIdx, 1, result);
            bfs(heights, flows, rows - 1, colIdx, 1 << 1, result);
        }

        for (int rowIdx = 0; rowIdx < rows; rowIdx++) {
            bfs(heights, flows, rowIdx, 0, 1, result);
            bfs(heights, flows, rowIdx, cols - 1, 1 << 1, result);
        }

        return result;
    }

    public void bfs(int[][] heights, int[][] flows, int rowIdx, int colIdx, int bit, List<List<Integer>> result) {
        if ((flows[rowIdx][colIdx] & bit) > 0) {
            return;
        }
        flows[rowIdx][colIdx] |= bit;
        if (flows[rowIdx][colIdx] == 3) {
            result.add(List.of(rowIdx, colIdx));
        }
        for (int[] direction : directions) {
            int newRowIdx = rowIdx + direction[0];
            int newColIdx = colIdx + direction[1];
            if (newRowIdx < 0 || newRowIdx >= flows.length ||
                    newColIdx < 0 || newColIdx >= flows[0].length) {
                continue;
            }
            if (heights[newRowIdx][newColIdx] < heights[rowIdx][colIdx]) {
                continue;
            }

            bfs(heights, flows, newRowIdx, newColIdx, bit, result);
        }
    }
}
