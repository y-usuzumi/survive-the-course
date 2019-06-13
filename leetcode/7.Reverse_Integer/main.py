class Solution:
    def reverse(self, x: int) -> int:
        result = 0
        if x > 2**31 - 1 or x < -2**31:
            return 0
        negative = False
        if x < 0:
            negative = True
            x = -x
        while x != 0:
            result *= 10
            result += x % 10
            x //= 10
        if negative:
            result = -result
        if result > 2**31 - 1 or result < -2**31:
            return 0
        return result

if __name__ == '__main__':
    sol = Solution()
    print(sol.reverse(123))  # 321
    print(sol.reverse(-123))  # -321
    print(sol.reverse(0))  # 0
    print(sol.reverse(10))  # 1
    print(sol.reverse(-2147483648))  # 0
