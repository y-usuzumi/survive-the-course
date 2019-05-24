class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        visited = {}
        curr_len = 0
        max_len = 0
        for idx, c in enumerate(s):
            last_idx = visited.get(c)
            if last_idx is None or idx - curr_len > last_idx:
                curr_len += 1
            else:
                if curr_len > max_len:
                    max_len = curr_len
                curr_len = idx - last_idx
            visited[c] = idx
        if curr_len > max_len:
            max_len = curr_len
        return max_len
