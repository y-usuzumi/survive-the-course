import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(self.sol.divide(10, 3), 3)

    def test_2(self):
        self.assertEqual(self.sol.divide(7, -3), -2)

    def test_3(self):
        self.assertEqual(self.sol.divide(-5, 2), -2)
