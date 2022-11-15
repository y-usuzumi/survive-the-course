package survive.the.course.java.leetcode._200_Number_of_Islands;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class SolutionTest {
    @Test
    public void testSolution() {
        var grid = new char[][]{
            {'1','1','1','1','1','0','1','1','1','1','1','1','1','1','1','0','1','0','1','1'},
            {'0','1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','0'},
            {'1','0','1','1','1','0','0','1','1','0','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','0','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','0','0','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','0','1','1','1','1','1','1','0','1','1','1','0','1','1','1','0','1','1','1'},
            {'0','1','1','1','1','1','1','1','1','1','1','1','0','1','1','0','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','0','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'0','1','1','1','1','1','1','1','0','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','0','1','1','1','1','1','1','1','0','1','1','1','1','1','1'},
            {'1','0','1','1','1','1','1','0','1','1','1','0','1','1','1','1','0','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','1','1','0'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','0','1','1','1','1','0','0'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'},
            {'1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'}};
        Solution solution = new Solution();
        assertEquals(1, solution.numIslands(grid));
    }
}