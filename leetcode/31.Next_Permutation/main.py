from typing import List

class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        l = len(nums)
        idx = l - 1
        while idx > 0:
            if nums[idx-1] < nums[idx]:
                # TODO: Optimize me
                n = nums[idx-1]
                nidx = idx
                while nidx < len(nums):
                    if nums[nidx] <= n:
                        nums[idx-1], nums[nidx-1] = nums[nidx-1], nums[idx-1]
                        break
                    nidx += 1
                else:
                    nums[idx-1], nums[nidx-1] = nums[nidx-1], nums[idx-1]
                break
            idx -= 1
        # Order from the peak index to the rest
        skip = idx - 1
        while idx <= (l + skip) // 2:
            ridx = l - idx + skip
            nums[idx], nums[ridx] = nums[ridx], nums[idx]
            idx += 1
        return
