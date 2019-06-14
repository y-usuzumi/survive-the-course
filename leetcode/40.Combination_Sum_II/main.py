from typing import List
from itertools import groupby

class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates = sorted(candidates)
        candidates = list((k, sum(1 for _ in g)) for k, g in groupby(candidates))
        return self._combination_sum2(candidates, 0, target, [])

    def _combination_sum2(self, candidates, idx, target, selected_nums):
        if target < 0:
            return []
        if target == 0:
            return [selected_nums.copy()]
        if idx >= len(candidates):
            return []

        result = []
        result.extend(self._combination_sum2(candidates, idx + 1, target, selected_nums))
        num, cnt = candidates[idx]
        curr_cnt = 0
        while curr_cnt < cnt:
            curr_cnt += 1
            selected_nums.append(num)
            result.extend(self._combination_sum2(candidates, idx + 1, target - num*curr_cnt, selected_nums))
        selected_nums[-1*cnt:] = []
        return result
