package survive.the.course.java.leetcode._1782_Count_Pairs_of_Nodes;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.Test;

public class SolutionTest {
    @Test
    public void test1() {
        int[] result = new Solution().countPairs(4, new int[][] { { 1, 2 }, { 2, 4 }, { 1, 3 }, { 2, 3 }, { 2, 1 } },
                new int[] { 1, 2, 3, 4 });
        assertArrayEquals(new int[] { 6, 6, 5, 2 }, result);
    }

    @Test
    public void test2() {
        int[] result = new Solution().countPairs(12, new int[][] { { 6, 11 } },
                new int[] { 0, 0, 0, 0 });
                assertArrayEquals(new int[] { 21, 21, 21, 21 }, result);
    }

    @Test
    public void test3() {
        int[] result = new Solution().countPairs(2, new int[][] { { 1, 2 }, { 1, 2 },{ 1, 2 },{ 1, 2 }},
                new int[] { 0 });
                assertArrayEquals(new int[] { 1 }, result);
    }
}