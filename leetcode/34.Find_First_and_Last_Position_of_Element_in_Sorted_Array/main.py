from typing import List

class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        l = len(nums)
        if l == 0:
            return [-1, -1]
        minbound, maxbound = 0, l

        # Find start
        while minbound < maxbound:
            idx = (minbound + maxbound) // 2
            if nums[idx] < target:
                minbound = idx + 1
            elif nums[idx] >= target:
                maxbound = idx
        if minbound >= l:
            return [-1, -1]
        if minbound < l and nums[minbound] == target:
            left = minbound
        elif minbound + 1 < l and nums[minbound + 1] == target:
            left = minbound + 1
            minbound += 1
        else:
            return [-1, -1]

        # Find end
        maxbound = len(nums)
        while minbound < maxbound:
            idx = (minbound + maxbound) // 2
            if nums[idx] <= target:
                minbound = idx + 1
            elif nums[idx] > target:
                maxbound = idx
        if minbound < l and nums[minbound] == target:
            right = minbound
        elif nums[minbound - 1] == target:
            right = minbound - 1
        else:
            return [-1, -1]

        return [left, right]

