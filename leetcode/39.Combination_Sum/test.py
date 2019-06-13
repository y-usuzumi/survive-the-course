import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            sorted(self.sol.combinationSum([2, 3, 6, 7], 7)),
            sorted([
                [7],
                [2, 2, 3]
            ])
        )
