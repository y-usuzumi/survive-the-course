# I             1
# V             5
# X             10
# L             50
# C             100
# D             500
# M             1000

_symbols = {
    'M': 1000,
    'D': 500,
    'C': 100,
    'L': 50,
    'X': 10,
    'V': 5,
    'I': 1,
}

class Solution:
    def romanToInt(self, s: str) -> int:
        result = 0
        l = len(s)
        for idx, ch in enumerate(s):
            num = _symbols[ch]
            if idx + 1 < l and _symbols[s[idx+1]] > num:
                result -= num
            else:
                result += num
        return result


if __name__ == '__main__':
    sol = Solution()
    print(sol.romanToInt("III"))  # 3
    print(sol.romanToInt("IV"))  # 4
    print(sol.romanToInt("IX"))  # 9
    print(sol.romanToInt("LVIII"))  # 58
    print(sol.romanToInt("MCMXCIV"))  # 1994
