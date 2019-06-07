class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        idx = 0
        l = len(nums)
        past = 0
        while idx < l:
            if idx+1 < l and nums[idx+1] == nums[idx]:
                past += 1
            elif past > 0:
                nums[idx-past] = nums[idx]
            idx += 1
        return l - past


if __name__ == '__main__':
    sol = Solution()
    lst = [1, 1, 2, 3, 4, 4, 4, 4, 5, 6, 6]
    print(sol.removeDuplicates(lst))
    print(lst)
