package survive.the.course.java.leetcode._1371_Find_the_Longest_Substring_Containing_Vowels_in_Even_Counts;

import java.util.HashMap;

class Solution {
    // This problem is very similar to https://leetcode.com/problems/subarray-sums-divisible-by-k/submissions/
    // Prefix sum
    public int findTheLongestSubstring(String s) {
        HashMap<Integer, Integer> occurrenceToIndexMap = new HashMap<>();
        // This is so we don't miss the edge case where the substring starts
        // from the beginning (no vowels before that)
        occurrenceToIndexMap.put(0, -1);
        int occurrence = 0;
        int result = 0;
        for (int idx = 0; idx < s.length(); idx++) {
            char ch = s.charAt(idx);
            int indexOfVowel = getIndexOfVowel(ch);
            if (indexOfVowel > -1) {
                occurrence ^= 1 << getIndexOfVowel(ch);
            }
            if (occurrenceToIndexMap.containsKey(occurrence)) {
                result = Math.max(result, idx - occurrenceToIndexMap.get(occurrence));
            } else {
                occurrenceToIndexMap.put(occurrence, idx);
            }
        }

        return result;
    }

    int getIndexOfVowel(char ch) {
        switch (ch) {
            case 'a':
            return 0;
            case 'e':
            return 1;
            case 'i':
            return 2;
            case 'o':
            return 3;
            case 'u':
            return 4;
        }
        return -1;
    }
}
