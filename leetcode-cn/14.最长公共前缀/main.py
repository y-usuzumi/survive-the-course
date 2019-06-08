from typing import List

class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if len(strs) == 0:
            return ""
        if len(strs) == 1:
            return strs[0]
        common_prefix = strs[0]
        l_common_prefix = len(common_prefix)
        s_idx = 1
        while s_idx < len(strs) and common_prefix:
            s = strs[s_idx]
            ls = len(s)
            l_min = min(l_common_prefix, ls)
            idx = 0
            while idx < l_min and s[idx] == common_prefix[idx]:
                idx += 1
            common_prefix = s[:idx]
            l_common_prefix = len(common_prefix)
            s_idx += 1
        return common_prefix
