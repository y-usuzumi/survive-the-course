import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(self.sol.twoSum([2, 7, 11, 15], 9), [0, 1])
