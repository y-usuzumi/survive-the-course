from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        memo = {}
        for idx, num in enumerate(nums):
            diff = target - num
            mem = memo.get(diff)
            if mem is not None:
                return [mem, idx]
            memo[num] = idx
        return None
