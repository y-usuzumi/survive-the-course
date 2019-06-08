class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        solutions = list(self._solve(0, n, '', 0))
        return solutions

    def _solve(self, n, max_n, ops, stack_lvl):
        if stack_lvl < 0:
            return
        if n > max_n:
            return

        if n == max_n:
            yield ops + ')' * stack_lvl
            return

        yield from self._solve(n, max_n, ops + ')', stack_lvl-1)
        yield from self._solve(n+1, max_n, ops + '(', stack_lvl+1)
