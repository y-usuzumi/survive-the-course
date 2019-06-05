from itertools import groupby

class Solution:
    def countAndSay(self, n: int) -> str:
        result = "1"
        if n == 1:
            return result
        idx = 1
        while idx < n:
            result = self._say(result)
            idx += 1
        return result

    def _say(self, s):
        result = []
        for (k, g) in groupby(s):
            result.append(str(len(list(g))) + k)
        return ''.join(result)




if __name__ == '__main__':
    sol = Solution()
    print(sol.countAndSay(1))  # 1
    print(sol.countAndSay(4))  # 1211
