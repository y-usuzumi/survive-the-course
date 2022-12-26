// https://leetcode.com/problems/make-sum-divisible-by-p/

package survive.the.course.java.leetcode._1590_Make_Sum_Divisible_by_P;

import java.util.HashMap;

public class Solution {
    public int minSubarray(int[] nums, int p) {
        int mod = 0;
        for (int idx = 0; idx < nums.length; idx++) {
            mod += nums[idx];
            mod %= p;
        }

        if (mod == 0) {
            return 0;
        }

        HashMap<Integer, Integer> indexMap = new HashMap<>();
        indexMap.put(0, -1);
        int result = Integer.MAX_VALUE;

        int currMod = 0;
        for (int idx = 0; idx < nums.length; idx++) {
            currMod += nums[idx];
            currMod %= p;
            int complementary = getComplementaryMod(currMod, mod, p);
            Integer complementaryIdx = indexMap.get(complementary);
            if (complementaryIdx != null) {
                result = Math.min(result, idx - complementaryIdx);
            }
            indexMap.put(currMod, idx);
        }

        if (result == Integer.MAX_VALUE || result == nums.length) {
            return -1;
        }

        return result;
    }

    private int getComplementaryMod(int currMod, int sumMod, int p) {
        int result = currMod - sumMod;
        if (result < 0) {
            return p + result;
        }
        return result;
    }
}
