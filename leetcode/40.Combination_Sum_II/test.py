import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            sorted(self.sol.combinationSum2([10, 1, 2, 7, 6, 1, 5], 8)),
            sorted([
                [1, 7],
                [1, 2, 5],
                [2, 6],
                [1, 1, 6]
            ])
        )
