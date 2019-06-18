import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        )
