# 这是个非常弱智的算法。
# 更好的算法请参见：
#
# KMP算法（克努斯-莫里斯-普拉特算法）：https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
# Boyer-Moore算法：https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm


class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if not needle:
            return 0
        lh, ln = len(haystack), len(needle)
        idx, max_idx = 0, lh - ln
        cmp_idx, cmp_max_idx = 0, idx + ln - 1
        while idx <= max_idx:
            cmp_idx = idx
            cmp_max_idx += 1
            while cmp_idx < cmp_max_idx:
                if haystack[cmp_idx] != needle[cmp_idx - idx]:
                    break
                cmp_idx += 1
            else:
                return idx
            idx += 1
        if idx > max_idx:
            return -1


