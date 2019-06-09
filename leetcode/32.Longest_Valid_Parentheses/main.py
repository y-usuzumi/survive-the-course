class Solution:
    def longestValidParentheses(self, s: str) -> int:
        stack = [-1]
        max_len = 0
        for idx, ch in enumerate(s):
            if ch == '(':
                stack.append(idx)
            else:
                stack.pop()
                if not stack:
                    stack.append(idx)
                curr_len = idx - stack[-1]
                if curr_len > max_len:
                    max_len = curr_len
        return max_len
