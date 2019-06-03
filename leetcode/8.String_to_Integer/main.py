class Solution:
    def myAtoi(self, str: str) -> int:
        result = 0
        l = len(str)
        idx = 0
        negative = False
        while idx < l:
            if str[idx] != ' ':
                break
            idx += 1
        if idx >= len(str):
            return result
        if str[idx] == '-':
            negative = True
            idx += 1
        elif str[idx] == '+':
            idx += 1
        while idx < l and str[idx] >= '0' and str[idx] <= '9':
            result *= 10
            result += ord(str[idx]) - ord('0')
            idx += 1
        if negative:
            result = -result
        if result > 2 ** 31 - 1:
            return 2 ** 31 - 1
        if result < -2 ** 31:
            return -2 ** 31
        return result


if __name__ == '__main__':
    sol = Solution()
    print(sol.myAtoi("42"))
    print(sol.myAtoi("-42"))
    print(sol.myAtoi("   42"))
    print(sol.myAtoi("sdlfk42"))
    print(sol.myAtoi("+42"))
