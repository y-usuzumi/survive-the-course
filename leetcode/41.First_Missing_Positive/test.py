import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.firstMissingPositive([1, 2, 0]),
            3
        )

    def test_2(self):
        self.assertEqual(
            self.sol.firstMissingPositive([3, 4, -1, 1]),
            2
        )

    def test_3(self):
        self.assertEqual(
            self.sol.firstMissingPositive([7, 8, 9, 11, 12]),
            1
        )

    def test_4(self):
        self.assertEqual(
            self.sol.firstMissingPositive([1]),
            2
        )

    def test_5(self):
        self.assertEqual(
            self.sol.firstMissingPositive([2, 1]),
            3
        )

    def test_6(self):
        self.assertEqual(
            self.sol.firstMissingPositive([2, 2]),
            1
        )
