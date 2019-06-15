import unittest
from main import Graph


class FloydTest(unittest.TestCase):
    def test_1(self):
        g = Graph(
            ['A', 'B', 'C', 'D', 'E', 'F'],
            [
                (0, 1, 10),
                (0, 2, 8),
                (1, 2, 1),
                (1, 3, 30),
                (1, 4, 6),
                (2, 4, 20),
                (3, 4, 7),
                (4, 5, 15)
            ]
        )
        print(g.floyd())
