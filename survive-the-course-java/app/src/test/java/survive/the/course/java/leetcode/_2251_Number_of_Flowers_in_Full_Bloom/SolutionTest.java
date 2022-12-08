package survive.the.course.java.leetcode._2251_Number_of_Flowers_in_Full_Bloom;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.Test;

public class SolutionTest {
    @Test
    public void test1() {
        int[] result = new Solution().fullBloomFlowers(new int[][] { { 1, 6 }, { 3, 7 }, { 9, 12 }, { 4, 13 } },
                new int[] { 2, 3, 7, 11 });
        assertArrayEquals(new int[] { 1, 2, 2, 2 }, result);
    }

    @Test
    public void test2() {
        int[] result = new Solution().fullBloomFlowers(new int[][] { { 1, 10 }, { 3, 3 } },
                new int[] { 3, 3, 2 });
        assertArrayEquals(new int[] { 2, 2, 1 }, result);
    }
}