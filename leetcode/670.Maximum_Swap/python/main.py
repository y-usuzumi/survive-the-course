# Although this approach has a time complexity of O(n),
# it has a space complexity of O(n). And it turns out to be slow.
# See main2.py for a better approach

from collections import defaultdict
from functools import reduce
import heapq


class Solution:
    def maximumSwap(self, num: int) -> int:
        digits = []
        while num > 0:
            digits.append(num % 10)
            num //= 10
        memo = defaultdict(list)
        occurred_digits_heap = []
        for idx, digit in enumerate(digits):
            if digit not in memo:
                heapq.heappush(occurred_digits_heap, -digit)
            memo[digit].append(idx)

        swap_indices = None
        while len(occurred_digits_heap) > 0:
            curr_digit = -heapq.heappop(occurred_digits_heap)
            compare_against = 0
            for idx in memo[curr_digit]:
                if idx > compare_against:
                    swap_indices = (compare_against, idx)
                compare_against += 1
        if swap_indices is not None:
            digits[swap_indices[0]], digits[swap_indices[1]] = digits[swap_indices[1]], digits[swap_indices[0]]
        return reduce(lambda l, r: l * 10 + r, digits, 0)
