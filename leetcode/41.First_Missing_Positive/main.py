from typing import List

class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        if not nums:
            return 1
        left_idx = 0
        right_idx = len(nums) - 1

        while left_idx <= right_idx:
            curr_num = nums[left_idx]
            if curr_num - 1 != left_idx:
                if curr_num < left_idx + 1 or curr_num > right_idx + 1:
                    nums[left_idx], nums[right_idx] = nums[right_idx], nums[left_idx]
                    right_idx -= 1
                else:
                    left, right = nums[left_idx], nums[curr_num-1]
                    if left != right:
                        nums[left_idx], nums[curr_num-1] = right, left
                    else:
                        nums[left_idx], nums[right_idx] = nums[right_idx], nums[left_idx]
                        right_idx -= 1
            else:
                left_idx += 1

        return left_idx + 1

