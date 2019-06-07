import unittest
from main import ListNode, Solution


def make_link_list(*vals):
    if not vals:
        return None
    start = curr = ListNode(vals[0])
    for idx in range(1, len(vals)):
        curr.next = ListNode(vals[idx])
        curr = curr.next
    return start


def display_link_list(node):
    from io import StringIO
    vals = []
    while node is not None:
        vals.append(node.val)
        node = node.next
    return " -> ".join([str(val) for val in vals])


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        l1 = make_link_list(2, 4, 3)
        l2 = make_link_list(5, 6, 4)
        self.assertEqual(
            display_link_list(self.sol.addTwoNumbers(l1, l2)),
            "7 -> 0 -> 8"
        )

    def test_2(self):
        l1 = make_link_list(0)
        l2 = make_link_list(0)
        self.assertEqual(
            display_link_list(self.sol.addTwoNumbers(l1, l2)),
            "0"
        )

    def test_3(self):
        l1 = make_link_list(9, 9, 9)
        l2 = make_link_list(1, 1, 1)
        self.assertEqual(
            display_link_list(self.sol.addTwoNumbers(l1, l2)),
            "0 -> 1 -> 1 -> 1"
        )
