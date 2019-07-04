# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x, left=None, right=None):
        self.val = x
        self.left = left
        self.right = right


class Solution:
    def isValidBST(self, root: TreeNode) -> bool:
        if root is None:
            return True
        return (
            self._isValidBST_L(root.left, root.val)
            and
            self._isValidBST_R(root.right, root.val)
        )

    def _isValidBST_L(self, root: TreeNode, max_num) -> bool:
        if root is None:
            return True
        if root.val >= max_num:
            return False
        return (
            self._isValidBST_L(root.left, root.val)
            and
            self._isValidBST_LR(root.right, root.val, max_num)
        )

    def _isValidBST_R(self, root: TreeNode, min_num) -> bool:
        if root is None:
            return True
        if root.val <= min_num:
            return False
        return (
            self._isValidBST_R(root.right, root.val)
            and
            self._isValidBST_LR(root.left, min_num, root.val)
        )

    def _isValidBST_LR(self, root: TreeNode, min_num, max_num) -> bool:
        if root is None:
            return True
        if root.val <= min_num or root.val >= max_num:
            return False
        return (
            self._isValidBST_LR(root.left, min_num, root.val)
            and
            self._isValidBST_LR(root.right, root.val, max_num)
        )
