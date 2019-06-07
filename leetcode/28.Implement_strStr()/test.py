import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(self.sol.strStr("hello", "ll"), 2)

    def test_2(self):
        self.assertEqual(self.sol.strStr("aaaaa", "bba"), -1)
