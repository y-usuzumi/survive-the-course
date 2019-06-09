import unittest
from main import Solution


class SolutionTest(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_1(self):
        self.assertEqual(
            self.sol.findSubstring(
                "barfoothefoobarman",
                ["foo", "bar"]
            ),
            [0, 9]
        )

    def test_2(self):
        self.assertEqual(
            self.sol.findSubstring(
                "wordgoodgoodgoodbestword",
                ["word", "good", "best", "word"]
            ),
            []
        )

    def test_3(self):
        self.assertEqual(
            self.sol.findSubstring(
                "barfoofoobarthefoobarman",
                ["bar","foo","the"]
            ),
            [6, 9, 12]
        )

    def test_4(self):
        self.assertEqual(
            self.sol.findSubstring(
                "aaaaaaaa",
                ["aa","aa","aa"]
            ),
            [0, 1, 2]
        )
