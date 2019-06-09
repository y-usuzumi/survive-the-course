import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.search([4, 5, 6, 7, 0, 1, 2], 0),
            4
        )

    def test_2(self):
        self.assertEqual(
            self.sol.search([4, 5, 6, 7, 0, 1, 2], 3),
            -1
        )
