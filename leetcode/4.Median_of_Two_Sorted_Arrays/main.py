import os
import json
from enum import IntEnum
from typing import *

class Reason(IntEnum):
    nums2_out_of_minbound = 1
    nums2_out_of_maxbound = 2

class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        l1, l2 = len(nums1), len(nums2)
        if l1 > l2:
            nums1, nums2 = nums2, nums1
            l1, l2 = l2, l1
        half_len = (l1 + l2) // 2
        minbound, maxbound, idx1 = 0, l1, l1 // 2
        while True:
            idx2 = half_len - idx1
            if l1 == 0:
                break
            if idx1 > 0 and nums1[idx1-1] > nums2[idx2]:
                idx1, maxbound = (minbound + maxbound) // 2, idx1
            elif idx2-1 > 0 and idx1 < l1 and nums2[idx2-1] > nums1[idx1]:
                idx1, minbound = (minbound + l1 + 1) // 2, idx1
            else:
                break

        print(f"INDICES: ({idx1}, {idx2})")

        if (l1 + l2) % 2 == 0:  # Two medians
            if l1 == 0:
                return (nums2[idx2-1] + nums2[idx2]) / 2
            if idx1 == 0:
                if idx2 >= l2:
                    right_min = nums1[idx1]
                else:
                    right_min = min(nums2[idx2], nums1[idx1])
                return (right_min + nums2[idx2-1]) / 2
            if idx1 >= l1:
                return (max(nums1[-1], nums2[idx2-1]) + nums2[idx2]) / 2
            else:
                return (max(nums1[idx1-1], nums2[idx2-1]) + min(nums1[idx1], nums2[idx2])) / 2
        else:
            if l1 == 0:
                return nums2[idx2]
            if idx1 >= l1:
                right_min = nums2[idx2]
            else:
                right_min = min(max(nums1[idx1], nums2[idx2-1]), nums2[idx2])
            return right_min
        print(f"VALUES: ({nums1[idx1]}, {nums2[idx2]})")

    def solve_and_print_result(self, nums1, nums2):
        print("Finding median for %s and %s" % (nums1, nums2))
        result = self.findMedianSortedArrays(nums1, nums2)
        print("Result: %s" % result)

if __name__ == '__main__':
    s = Solution()

    nums1 = [1, 2, 3, 5, 6, 7]
    nums2 = [2, 4, 6, 7, 9, 10]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1, 2, 3, 5, 6]
    nums2 = [2, 4, 6, 7, 9, 10]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1, 2, 3, 4, 5]
    nums2 = [6, 7, 8, 9, 10]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [6, 7, 8, 9, 10]
    nums2 = [1, 2, 3, 4, 5]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [6, 7, 8, 9, 10, 11, 12, 13]
    nums2 = [1, 2, 3, 4, 5]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [6, 7]
    nums2 = [1, 2, 3, 4, 5, 6, 7, 8]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1]
    nums2 = [10]
    s.solve_and_print_result(nums1, nums2)

    nums1 = []
    nums2 = [5]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1, 2, 3]
    nums2 = []
    s.solve_and_print_result(nums1, nums2)

    nums1 = []
    nums2 = [2, 3]
    s.solve_and_print_result(nums1, nums2)

    nums1 = []
    nums2 = [1, 2, 3, 4]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1]
    nums2 = [2, 3, 4, 5, 6]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1, 2]
    nums2 = [3, 4]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1]
    nums2 = [2, 3]
    s.solve_and_print_result(nums1, nums2)

    nums1 = [1]
    nums2 = [2, 3, 4, 5]
    s.solve_and_print_result(nums1, nums2)

    with open(os.path.join(os.path.dirname(__file__), 'fixtures', 't1.data')) as f:
        import json
        nums1 = json.loads(f.readline())
        nums2 = json.loads(f.readline())
        s.solve_and_print_result(nums1, nums2)
