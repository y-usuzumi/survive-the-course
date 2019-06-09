from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if not nums:
            return -1
        l = len(nums)
        if l == 1:
            return 0 if target == nums[0] else -1
        pv = nums[-1]
        minbound, maxbound = 0, l-1
        while minbound < maxbound:
            idx = (minbound + maxbound) // 2
            if nums[idx] < pv:
                maxbound = idx
            else:
                minbound = idx+1
        if target <= pv:  # right part
            minbound, maxbound = minbound, l
        else:  # left part
            minbound, maxbound = 0, minbound
        while minbound < maxbound:
            idx = (minbound + maxbound) // 2
            if nums[idx] < target:
                minbound = idx+1
            elif nums[idx] > target:
                maxbound = idx
            else:
                return idx
        if nums[minbound] != target:
            return -1

