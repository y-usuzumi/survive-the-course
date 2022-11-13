package survive.the.course.java.leetcode._200_Number_of_Islands;

import java.util.ArrayDeque;

/**
 * This solution uses BFS to scan through contiguous locations and mark them as visited. Whenever
 * we move to a location which is "land" and not visited, it is a new island.
 */
public class Solution2 {
    private static final int[][] directions = new int[][]{{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

    public int numIslands(char[][] grid) {
        if (grid.length == 0 || grid[0].length == 0) {
            return 0;
        }
        int rows = grid.length;
        int cols = grid[0].length;
        boolean[][] visited = new boolean[rows][];
        for (int rowIdx=0; rowIdx < rows; rowIdx++) {
            visited[rowIdx] = new boolean[cols];
        }

        int result = 0;

        for (int rowIdx=0; rowIdx < rows; rowIdx++) {
            for (int colIdx = 0; colIdx < cols; colIdx++) {
                if (grid[rowIdx][colIdx] == '0' || visited[rowIdx][colIdx]) {
                    continue;
                }
                result++;
                bfs(grid, visited, rowIdx, colIdx);
            }
        }

        return result;
    }

    private void bfs(char[][] grid, boolean[][] visited, int rowIdx, int colIdx) {
        ArrayDeque<Integer> qRow = new ArrayDeque<>();
        ArrayDeque<Integer> qCol = new ArrayDeque<>();
        qRow.add(rowIdx);
        qCol.add(colIdx);
        visited[rowIdx][colIdx] = true;
        while (!qRow.isEmpty()) {
            int currRowIdx = qRow.removeFirst();
            int currColIdx = qCol.removeFirst();
            for (int[] direction : directions) {
                int nextRowIdx = currRowIdx + direction[0];
                int nextColIdx = currColIdx + direction[1];
                if (nextRowIdx < 0 || nextColIdx < 0 || nextRowIdx >= grid.length || nextColIdx >= grid[0].length) {
                    continue;
                }
                if (visited[nextRowIdx][nextColIdx]) {
                    continue;
                }
                if (grid[nextRowIdx][nextColIdx] == '0') {
                    continue;
                }
                // System.out.println(String.format("Adding (%d, %d)", nextRowIdx, nextColIdx));
                qRow.add(nextRowIdx);
                qCol.add(nextColIdx);
                visited[nextRowIdx][nextColIdx] = true;
            }
        }
    }
}
