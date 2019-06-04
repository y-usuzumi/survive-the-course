from typing import *

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums = sorted(nums)
        idx1 = 0
        l = len(nums)
        closest = None
        closest_dist = 2 ** 31 - 1
        has_eq = False
        while idx1 < l - 2:
            idx2, idx3 = idx1 + 1, l - 1
            while idx1 > 0 and idx1 < idx2:
                if nums[idx1] != nums[idx1 - 1]:
                    break
                idx1 += 1
            while idx2 < idx3:
                s = nums[idx2] + nums[idx3] + nums[idx1]

                dist = abs(s - target)
                if dist < closest_dist:
                    closest = s
                    closest_dist = dist
                if s > target:
                    while idx3 > idx2 and nums[idx3-1] == nums[idx3]:
                        idx3 -= 1
                    idx3 -= 1
                elif s < target:
                    while idx2 < idx3 and nums[idx2] == nums[idx2+1]:
                        idx2 += 1
                    idx2 += 1
                else:
                    break
            if has_eq:
                break
            idx1 += 1
        return closest


if __name__ == '__main__':
    sol = Solution()
    print(sol.threeSumClosest([-1, 2, 1, -4], 1))  # 2 (-1 + 2 + 1 == 2)
    print(sol.threeSumClosest([0, 1, 2], 3))  # 3
    print(sol.threeSumClosest([1, 1, 1, 1], -100))
