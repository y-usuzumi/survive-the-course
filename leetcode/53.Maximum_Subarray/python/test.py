import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.maxSubArray([-2,1,-3,4,-1,2,1,-5,4]),
            6
        )
