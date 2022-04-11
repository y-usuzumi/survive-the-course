import unittest
from problems._19_remove_nth_node_from_end_of_list import ListNode, Solution

class Test(unittest.TestCase):
    def test_1(self):
        head = ListNode.from_vals([1, 2, 3, 4, 5])
        new_head = Solution().removeNthFromEnd(head, 2)
        self.assertEqual([1, 2, 3, 5], new_head.to_vals())

    def test_2(self):
        head = ListNode.from_vals([1])
        new_head = Solution().removeNthFromEnd(head, 1)
        self.assertIsNone(new_head)

    def test_3(self):
        head = ListNode.from_vals([1, 2])
        new_head = Solution().removeNthFromEnd(head, 1)
        self.assertEqual([1], new_head.to_vals())