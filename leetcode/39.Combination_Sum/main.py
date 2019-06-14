from typing import List
from itertools import chain

class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        results = []
        idx = 0
        l = len(candidates)
        ingreds = [(0, [])]
        cand_idx = 0
        while cand_idx < l:
            new_ingreds = []
            cand = candidates[cand_idx]
            for ingred_sum, ingred in ingreds:
                next_ingred_sum = ingred_sum
                cnt = 0
                while True:
                    if next_ingred_sum < target:
                        next_ingred = ingred + [cnt]
                        new_ingreds.append((next_ingred_sum, next_ingred))
                        next_ingred_sum += cand
                        cnt += 1
                    elif next_ingred_sum == target:
                        next_ingred = ingred + [cnt]
                        result = []
                        for idx, cnt in enumerate(next_ingred):
                            result += [candidates[idx]] * cnt
                        results.append(result)
                        break
                    else:
                        break
            ingreds = new_ingreds
            cand_idx += 1
        return results

