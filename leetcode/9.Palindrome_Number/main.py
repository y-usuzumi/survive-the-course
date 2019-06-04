class Solution:
    def isPalindrome(self, x: int) -> bool:
        old_x = x
        result = 0
        while x > 0:
            result *= 10
            result += x % 10
            x //= 10
        return result == old_x


if __name__ == '__main__':
    sol = Solution()
    print(sol.isPalindrome(121))  # True
    print(sol.isPalindrome(-121))  # False
    print(sol.isPalindrome(10))  # False
