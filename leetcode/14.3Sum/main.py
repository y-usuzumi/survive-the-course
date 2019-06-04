from typing import *

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums = sorted(nums)
        result = []
        l = len(nums)
        idx1 = 0
        while idx1 < l - 2:
            num1 = nums[idx1]
            while idx1 > 0 and idx1 < l - 2 and num1 == nums[idx1-1]:
                idx1 += 1
                num1 = nums[idx1]
            if num1 > 0:
                break
            idx2, idx3 = idx1+1, l-1
            while idx2 < idx3:
                num2 = nums[idx2]
                num3 = nums[idx3]
                sum23 = num2 + num3
                if sum23 == -num1:
                    result.append([num1, num2, num3])
                    while idx2 < idx3 and nums[idx2+1] == nums[idx2]:
                        idx2 += 1
                    idx2 += 1
                    while idx3 > idx2 and nums[idx3-1] == nums[idx3]:
                        idx3 -= 1
                    idx3 -= 1
                elif sum23 > -num1:
                    idx3 -= 1
                else:
                    idx2 += 1
            idx1 += 1
        return result



if __name__ == '__main__':
    sol = Solution()
    print(sol.threeSum([-1, 0, 1, 2, -1, -4]))
    print(sol.threeSum([0, 0, 0, 0]))

