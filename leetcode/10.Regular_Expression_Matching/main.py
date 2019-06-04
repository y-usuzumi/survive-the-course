class Solution:
    def get_stm(self, p:str) -> dict:
        start = {}
        idx = 0
        l = len(p)
        curr_state = start
        while idx < l:
            ch = p[idx]
            if ch in curr_state:
                next_state = {}
                curr_state[''] = next_state
            elif idx+1 < l and p[idx+1] == '*':
                next_state = {}
                next_state[ch] = next_state
                curr_state[''] = next_state
                idx += 2
            else:
                next_state = {}
                curr_state[ch] = next_state
                idx += 1
            curr_state = next_state
        curr_state['_FIN'] = True
        return start

    def _run_stm(self, stm, s, idx) -> bool:
        while idx < len(s):
            ch = s[idx]
            next_stms = []
            if ch in stm:
                next_stms.append((stm[ch], idx+1))
            if '.' in stm:
                next_stms.append((stm['.'], idx+1))
            if '' in stm:
                next_stms.append((stm[''], idx))
            if len(next_stms) == 1:
                stm, idx = next_stms[0]
                continue
            else:
                for xstm, next_idx in next_stms:
                    result = self._run_stm(xstm, s, next_idx)
                    if result:
                        return True
                return False

        if '_FIN' in stm:
            return True

        while '' in stm:
            stm = stm['']
            if '_FIN' in stm:
                return True

        return False

    def isMatch(self, s: str, p: str) -> bool:
        stm = self.get_stm(p)
        return self._run_stm(stm, s, 0)


if __name__ == '__main__':
    sol = Solution()
    print(sol.isMatch("aa", "a"))  # False
    print(sol.isMatch("aa", "a*"))  # True
    print(sol.isMatch("ab", ".*"))  # True
    print(sol.isMatch("aab", "c*a*b"))  # True
    print(sol.isMatch("mississippi", "mis*is*p*."))  # False
    print(sol.isMatch("ab", ".*c"))  # False
    print(sol.isMatch("aaa", "a*a"))  # True
    print(sol.isMatch("a", "ab*"))  # True
