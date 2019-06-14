import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.searchRange([5, 7, 7, 8, 8, 10], 8),
            [3, 4]
        )

    def test_2(self):
        self.assertEqual(
            self.sol.searchRange([5, 7, 7, 8, 8, 10], 6),
            [-1, -1]
        )

    def test_3(self):
        self.assertEqual(
            self.sol.searchRange([1], 1),
            [0, 0]
        )

    def test_4(self):
        self.assertEqual(
            self.sol.searchRange([2, 2], 3),
            [-1, -1]
        )
