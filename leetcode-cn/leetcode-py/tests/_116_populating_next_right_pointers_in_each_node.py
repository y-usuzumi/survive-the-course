import unittest


class Test(unittest.TestCase):
    @unittest.skip("Assertion too cumbersome to implement.")
    def test_1(self):
        raise NotImplementedError("Assertion too cumbersome to implement.")