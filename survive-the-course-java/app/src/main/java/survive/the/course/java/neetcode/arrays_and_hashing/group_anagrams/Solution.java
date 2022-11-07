package survive.the.course.java.neetcode.arrays_and_hashing.group_anagrams;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Collectors;

public class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> tm = new HashMap<>();
        for (String str : strs) {
            String key = getKey(str);
            List<String> values;
            if (!tm.containsKey(key)) {
                values = new ArrayList<>();
                tm.put(key, values);
            } else {
                values = tm.get(key);
            }
            values.add(str);
        }
        return tm.values().stream().collect(Collectors.toList());
    }

    private String getKey(final String s) {
        byte[] key = new byte[26];
        for (int idx = 0; idx < s.length(); idx++) {
            key[s.charAt(idx) - 'a'] += 1;
        }
        return new String(key);
    }
}
