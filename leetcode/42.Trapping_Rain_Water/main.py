from typing import List

class Solution:
    def trap(self, height: List[int]) -> int:
        if len(height) < 2:
            return 0

        l = len(height)

        left, right = 0, l - 1
        max_left, max_right = height[left], height[right]
        result = 0
        while left < right:
            if max_left <= max_right:
                left += 1
                h = height[left]
                if max_left >= h:
                    result += max_left - h
                else:
                    max_left = h
            else:
                right -= 1
                h = height[right]
                if max_right >= h:
                    result += max_right - h
                else:
                    max_right = h
        return result
