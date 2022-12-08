// https://leetcode.com/problems/number-of-flowers-in-full-bloom/

package survive.the.course.java.leetcode._2251_Number_of_Flowers_in_Full_Bloom;

import java.util.TreeMap;

public class Solution {
    // An easier version of https://leetcode.com/problems/my-calendar-ii/.
    // Once we set values of difference in each keypoint, we can do an
    // aggregation from the left. This is unlike in My Calendar II where
    // we need to compare an incoming range on the fly.
    public int[] fullBloomFlowers(int[][] flowers, int[] persons) {
        TreeMap<Integer, Integer> tm = new TreeMap<>();
        for (int[] flower : flowers) {
            int start = flower[0];
            int end = flower[1];
            tm.put(start, tm.getOrDefault(start, 0) + 1);
            tm.put(end+1, tm.getOrDefault(end+1, 0) - 1);
        }
        int curr = 0;
        for (int key : tm.keySet()) {
            curr = tm.get(key) + curr;
            tm.put(key, curr);
        }

        int[] result = new int[persons.length];
        for (int idx = 0; idx < persons.length; idx++) {
            var val = tm.floorEntry(persons[idx]);
            if (val != null) {
                result[idx] = val.getValue();
            }
        }

        return result;
    }
}
