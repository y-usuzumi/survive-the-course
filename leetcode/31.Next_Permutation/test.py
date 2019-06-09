import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        l = [1, 2, 3]
        self.sol.nextPermutation(l)
        self.assertEqual(l, [1, 3, 2])

    def test_2(self):
        l = [1, 2, 3, 2, 1]
        self.sol.nextPermutation(l)
        self.assertEqual(l, [1, 3, 1, 2, 2])

    def test_3(self):
        l = [3, 2, 1, 1, 1]
        self.sol.nextPermutation(l)
        self.assertEqual(l, [1, 1, 1, 2, 3])
