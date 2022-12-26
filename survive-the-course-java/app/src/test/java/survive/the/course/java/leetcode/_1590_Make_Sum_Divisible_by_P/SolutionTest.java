package survive.the.course.java.leetcode._1590_Make_Sum_Divisible_by_P;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class SolutionTest {
    @Test
    public void test1() {
        assertEquals(1, new Solution().minSubarray(new int[] { 3, 1, 4, 2 }, 6));
    }

    @Test
    public void test2() {
        assertEquals(2, new Solution().minSubarray(new int[] { 6, 3, 5, 2 }, 9));
    }

    @Test
    public void test3() {
        assertEquals(7, new Solution()
                .minSubarray(new int[] { 8, 32, 31, 18, 34, 20, 21, 13, 1, 27, 23, 22, 11, 15, 30, 4, 2 }, 148));
    }
}
