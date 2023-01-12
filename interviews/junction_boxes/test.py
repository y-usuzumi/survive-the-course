import unittest
from main import orderedJunctionBoxes


class SolutionTest(unittest.TestCase):
    def setUp(self):
        pass

    def test_1(self):
        numberOfBoxes = 6
        boxList = [
            "ykc 82 01",
            "eo first qpx",
            "09z cat hamster",
            "06f 12 25 6",
            "az0 first qpx",
            "236 cat dog rabbit snake"
        ]
        result = orderedJunctionBoxes(numberOfBoxes, boxList)
        self.assertEqual(
            result,
            [
                "236 cat dog rabbit snake",
                "09z cat hamster",
                "az0 first qpx",
                "eo first qpx",
                "ykc 82 01",
                "06f 12 25 6",
            ]
        )

    def test_2(self):
        numberOfBoxes = 6
        boxList = [
            "ykc 82 01",
            "eo first qpx",
            "09z cat hamster",
            "06f 12 25 6",
            "az0 first qpx",
            "236 cat dog rabbit snake"
        ]
        result = orderedJunctionBoxes(numberOfBoxes, boxList)
        self.assertEqual(
            result,
            [
                "236 cat dog rabbit snake",
                "09z cat hamster",
                "az0 first qpx",
                "eo first qpx",
                "ykc 82 01",
                "06f 12 25 6",
            ]
        )

