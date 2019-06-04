from typing import *
from itertools import product

_map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"]

class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        sets = [_map[ord(digit) - ord('2')] for digit in digits]
        return list([''.join(comb) for comb in product(*sets)])


if __name__ == '__main__':
    sol = Solution()
    print(sol.letterCombinations("23"))
