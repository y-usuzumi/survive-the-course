package survive.the.course.java.leetcode._743_Network_Delay_Time;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class SolutionTest {
    @Test
    public void test1() {
        int result = new Solution().networkDelayTime(new int[][]{{2,1,1},{2,3,1},{3,4,1}}, 4, 2);
        assertEquals(2, result);
    }
}
