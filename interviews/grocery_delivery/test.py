import unittest
from main import minimumDistance


class SolutionTest(unittest.TestCase):
    def setUp(self):
        pass

    def test_1(self):
        grid = [
            [1, 0, 0],
            [1, 0, 0],
            [1, 9, 1]
        ]
        self.assertEqual(minimumDistance(3, 3, grid), 3)

    def test_2(self):
        grid = [
            [1, 1, 1, 1],
            [0, 1, 1, 1],
            [0, 1, 0, 1],
            [1, 1, 9, 1],
            [0, 0, 1, 1]
        ]
        self.assertEqual(minimumDistance(5, 4, grid), 5)

    def test_3(self):
        grid = [
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 9]
        ]
        self.assertEqual(minimumDistance(5, 4, grid), 7)

    def test_4(self):
        grid = [
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 0, 0],
            [1, 1, 0, 9]
        ]
        self.assertEqual(minimumDistance(5, 4, grid), -1)

    def test_5(self):
        grid = [
            [1, 1, 1, 1],
            [0, 0, 0, 1],
            [1, 1, 1, 1],
            [1, 0, 0, 0],
            [1, 1, 1, 9]
        ]
        self.assertEqual(minimumDistance(5, 4, grid), 13)

