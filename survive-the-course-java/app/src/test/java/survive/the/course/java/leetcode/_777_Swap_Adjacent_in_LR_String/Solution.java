package survive.the.course.java.leetcode._777_Swap_Adjacent_in_LR_String;

public class Solution {
    // `start` can transform into `end` IFF all of the following satisfy:
    // 1. All relative positions of 'L' and 'R' need to be the same in `start` and `end`
    // 2. Every 'L' in `start` can occur at the same position or earlier in `end`
    // 3. Every 'R' in 'end' can occur at the same position or earlier in `start
    public boolean canTransform(String start, String end) {
        char[] chars1 = start.toCharArray();
        char[] chars2 = end.toCharArray();
        if (chars1.length != chars2.length) {
            return false;
        }

        int length = chars1.length;
        int numL = 0;
        int numR = 0;

        for (int idx = 0; idx < length; idx++) {
            // an 'L' can occur earlier in `end`, so we check `end` first.
            if (chars2[idx] == 'L') {
                // but we need to make sure all 'L's are cancelled out before
                // we meet an 'R', otherwise the relative positions are not the same
                if (numR > 0) {
                    return false;
                }
                numL++;
            }

            // Same goes with 'R' except we check `start` first.
            if (chars1[idx] == 'R') {
                if (numL > 0) {
                    return false;
                }
                numR++;
            }

            if (chars1[idx] == 'L') {
                // This means an 'L' occurs too early in `start`, which is invalid.
                if (numL == 0) {
                    return false;
                }
                // Cancel out
                numL--;
            }

            if (chars2[idx] == 'R') {
                // This means an 'R' occurs too early in 'end', which is invalid.
                if (numR == 0) {
                    return false;
                }
                // Cancel out
                numR--;
            }
        }

        // All 'L's and 'R's need to be cancelled out.
        return numL == 0 && numR == 0;
    }
}
