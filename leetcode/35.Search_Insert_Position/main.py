from typing import List

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        size = len(nums)
        l, m, r = 0, size//2, size-1
        while l < r:
            v = nums[m]
            if v < target:
                l, m = m+1, (r + m + 1) // 2
            elif v > target:
                m, r = (m + l) // 2, m
            else:
                return m
        return (m+1) if nums[m] < target else m


if __name__ == '__main__':
    sol = Solution()
    print(sol.searchInsert([1, 3, 5, 6], 5))  # 2
    print(sol.searchInsert([1, 3, 5, 6], 2))  # 1
    print(sol.searchInsert([1, 3, 5, 6], 7))  # 4
    print(sol.searchInsert([1, 3, 5, 6], 0))  # 0
    print(sol.searchInsert([1, 3], 4))  # 2
