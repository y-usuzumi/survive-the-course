import unittest
from main import stairs


class SolutionTest(unittest.TestCase):
    def setUp(self):
        pass

    def test_1(self):
        self.assertEqual(stairs(3, [1, 2]), 3)

    def test_2(self):
        self.assertEqual(stairs(4, [1, 2]), 5)

    def test_3(self):
        self.assertEqual(stairs(1, [2]), 0)

    def test_4(self):
        self.assertEqual(stairs(2, [2]), 1)
