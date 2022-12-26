package survive.the.course.java.leetcode._424_Longest_Repeating_Character_Replacement;

public class Solution {
    public int characterReplacement(String s, int k) {
        // Counter for each upper-case letter
        int[] charCounters = new int[26];
        var left = 0;
        char[] chars = s.toCharArray();
        int currMax = 0;
        int result = 0;
        // Double pointer
        for (int right = 0; right < chars.length; right++) {
            char currChar = chars[right];
            charCounters[currChar - 'A']++;
            // Update the number of characters of the letter that occurs most
            // frequently in the substring.
            currMax = Math.max(currMax, charCounters[currChar - 'A']);
            // The reason why we do not need to update currMax is that:
            // once we have found a valid substring, we don't really care if we
            // break the validity of the substring by moving both pointers, as
            // long as we don't change the max length. We are only looking ahead
            // to see a longer *valid* substring whose max char may be
            // different.
            if (right - left + 1 - currMax > k) {
                charCounters[chars[left] - 'A']--;
                left++;
            } else {
                result += 1;
            }
        }
        return result;
    }
}
