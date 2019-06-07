class Solution:
    def divide(self, dividend: int, divisor: int) -> int:
        if dividend == 0:
            return 0
        negate = False
        if dividend < 0:
            dividend = -dividend
            negate = not negate
        if divisor < 0:
            divisor = -divisor
            negate = not negate
        if dividend < divisor:
            return 0

        old_divisor = divisor
        result = 1
        while divisor + divisor <= dividend:
            divisor += divisor
            result += result
        rem = dividend - divisor
        result = result + self.divide(rem, old_divisor) if not negate else -result - self.divide(rem, old_divisor)
        if result > 2 ** 31 - 1:
            return 2 ** 31 - 1
        if result < -2 ** 31:
            return -2 ** 31
        return result
