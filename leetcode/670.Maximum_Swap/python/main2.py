# I thought this would be faster than main.py, but turns out, no.

from functools import reduce


class Solution:
    def maximumSwap(self, num: int) -> int:
        if num < 10:
            return num

        digits = [int(c) for c in str(num)]

        curr_min = digits[0]
        for idx, digit in enumerate(digits):
            if digit <= curr_min:
                curr_min = digit
            else:
                break
        else:
            return num
        min_sentinal, max_sentinal = idx - 1, idx
        curr_max = digit
        l = len(digits)
        idx = max_sentinal + 1
        while idx < l:
            curr = digits[idx]
            if curr >= curr_max:
                curr_max = curr
                max_sentinal = idx
            idx += 1
        while min_sentinal >= 0:
            if digits[min_sentinal] >= curr_max:
                break
            min_sentinal -= 1
        min_sentinal += 1
        digits[min_sentinal], digits[max_sentinal] = digits[max_sentinal], digits[min_sentinal]
        return reduce(lambda l, r: l * 10 + r, digits, 0)
