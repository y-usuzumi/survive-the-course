class Solution(object):
    def removeElement(self, nums, val):
        """
        :type nums: List[int]
        :type val: int
        :rtype: int
        """
        idx = 0
        l = len(nums)
        past = 0
        while idx < l:
            if nums[idx] == val:
                past += 1
            elif past > 0:
                nums[idx-past] = nums[idx]
            idx += 1
        return l - past


if __name__ == '__main__':
    sol = Solution()
    lst = [3, 2, 2, 3]
    print(sol.removeElement(lst, 3))  # 2
    print(lst)  # [2, 2, 2, 3]
    lst = [0, 1, 2, 2, 3, 0, 4, 2]
    print(sol.removeElement(lst, 2))  # 5
    print(lst)  # [0, 1, 3, 0, 4, 0, 4, 2]
