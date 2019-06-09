from typing import List

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        l = len(nums)
        if l == 0:
            return None
        curr_sum = max_sum = nums[0]
        idx = 1
        while idx < l:
            num = nums[idx]
            if num < 0:
                if curr_sum < 0:
                    curr_sum = num
                else:
                    curr_sum += num
            elif curr_sum < 0:
                curr_sum = num
            else:
                curr_sum += num
            if curr_sum > max_sum:
                max_sum = curr_sum
            idx += 1
        return max_sum

