package survive.the.course.java.neetcode.arrays_and_hashing.group_anagrams;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.List;

import org.junit.jupiter.api.Test;

import com.google.common.collect.Lists;

public class SolutionTest {
    @Test
    public void test1() {
        Solution solution = new Solution();
        assertEquals(
            solution.groupAnagrams(new String[] { "eat", "tea", "tan", "ate", "nat", "bat" }),
            List.of(
                List.of("bat"),
                List.of("nat", "tan"),
                List.of("ate", "eat", "tea")));
    }
}
