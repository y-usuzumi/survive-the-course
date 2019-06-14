class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        if not s:
            return 0
        l = len(s)
        idx0 = l - 1
        while idx0 >= 0 and s[idx0] == ' ':
            idx0 -= 1
        idx = idx0
        while idx >= 0:
            if s[idx] == ' ':
                break
            idx -= 1
        return idx0 - idx
