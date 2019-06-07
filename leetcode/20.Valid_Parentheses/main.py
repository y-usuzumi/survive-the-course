class Solution(object):
    def isValid(self, s):
        """
        :type s: str
        :rtype: bool
        """
        stack = []
        for ch in s:
            if ch == '(' or ch == '[' or ch == '{':
                stack.append(ch)
            elif ch == ')':
                if not stack or stack[-1] != '(':
                    return False
                else:
                    stack.pop()
            elif ch == ']':
                if not stack or stack[-1] != '[':
                    return False
                else:
                    stack.pop()
            elif ch == '}':
                if not stack or stack[-1] != '{':
                    return False
                else:
                    stack.pop()
        return not stack

if __name__ == '__main__':
    sol = Solution()
    print(sol.isValid("()"))  # True
    print(sol.isValid("()[]{}"))  # True
    print(sol.isValid("(]"))  # False
    print(sol.isValid("([)]"))  # False
    print(sol.isValid("{[]}"))  # True
