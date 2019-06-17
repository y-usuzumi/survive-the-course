import unittest
from main import LRUCache


class LRUCacheTest(unittest.TestCase):
    def test_1(self):
        cache = LRUCache(2)

        cache.put(1, 1)
        cache.put(2, 2)
        self.assertEqual(cache.get(1), 1)        # returns 1
        cache.put(3, 3)                          # evicts key 2
        self.assertEqual(cache._curr_cnt, 2)
        self.assertFalse(2 in cache._cache)
        self.assertEqual(cache.get(2), -1)       # returns -1 (not found)
        cache.put(4, 4)                          # evicts key 1
        self.assertEqual(cache._curr_cnt, 2)
        self.assertFalse(1 in cache._cache)
        self.assertEqual(cache.get(1), -1)       # returns -1 (not found)
        self.assertEqual(cache.get(3), 3)        # returns 3
        self.assertEqual(cache.get(4), 4)        # returns 4
