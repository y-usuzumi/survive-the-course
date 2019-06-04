# Seems DP

# 思路：
# 从两边开始，将较短的一边向中间收紧，每次记录路过的最大面积。当两边界相遇时，返回最大面积。
#
# 逻辑：
# 将两边围成的面积记为max，如果将较长的一边向中间移动，新包络的面积一定小于之前的面积。
# 所以更大的面积只可能出现在将较小的一边向中间移动。


class Solution(object):
    def maxArea(self, height):
        """
        :type height: List[int]
        :rtype: int
        """
        if len(height) < 2:
            return 0

        i, j = 0, len(height) - 1
        maxhl, maxhr = height[i], height[j]
        maxArea = min(maxhl, maxhr) * (j - i)
        while i < j:
            hl, hr = height[i], height[j]
            area = min(hl, hr) * (j - i)
            if area > maxArea:
                maxArea = area
            if hl < hr:
                i += 1
                continue
            else:
                j -= 1
                continue
        return maxArea


if __name__ == '__main__':
    sol = Solution()
    print(sol.maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7]))  # 49
