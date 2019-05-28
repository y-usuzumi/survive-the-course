class Solution:
    def maximumSwap(self, num: int) -> int:
        if num < 10:
            return num
        digits = [c for c in str(num)]
        max_idx, max_digit = 0, digits[0]
        for idx, digit in enumerate(digits):
            if digit >= max_digit:
                max_idx, max_digit = idx, digit

        for idx, digit in enumerate(digits):
            if digit < max_digit:
                break

        if idx < max_idx:
            digits[idx] = max_digit
            digits[max_idx] = digit
        return int(''.join(digits))
