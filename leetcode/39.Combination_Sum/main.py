from typing import List
from collections import defaultdict

class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        results = []
        idx = 0
        l = len(candidates)
        ingreds = [(0, [])]
        cand_idx = 0
        while cand_idx < l:
            new_ingreds = []
            for ingred_sum, ingred in ingreds:
                cand = candidates[cand_idx]
                cnt = 0
                while True:
                    next_ingred_sum = ingred_sum + cand * cnt
                    if next_ingred_sum < target:
                        next_ingred = ingred + [cand] * cnt
                        new_ingreds.append((next_ingred_sum, next_ingred))
                        cnt += 1
                    elif next_ingred_sum == target:
                        next_ingred = ingred + [cand] * cnt
                        results.append(next_ingred)
                        break
                    else:
                        break
            ingreds = new_ingreds
            cand_idx += 1
        return results

