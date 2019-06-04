# Symbol       Value
# I             1
# V             5
# X             10
# L             50
# C             100
# D             500
# M             1000

import io

_symbols = {
    1000: 'M',
    500: 'D',
    100: 'C',
    50: 'L',
    10: 'X',
    5: 'V',
    1: 'I'
}

class Solution:
    def intToRoman(self, num: int) -> str:
        sio = io.StringIO()
        base = 1000
        while base > 0:
            num = self.write_and_extract_rem(sio, num, base)
            base //= 10
        return sio.getvalue()

    def write_and_extract_rem(self, sio, num, base):
        num, rem = num // base, num % base
        base_symbol = _symbols[base]
        if num <= 3:
            sio.write(num * base_symbol)
        elif num == 4:
            sio.write(base_symbol)
            sio.write(_symbols[base * 5])
        elif num < 9:
            sio.write(_symbols[base * 5])
            sio.write((num - 5) * base_symbol)
        elif num == 9:
            sio.write(base_symbol)
            sio.write(_symbols[base * 10])
        return rem


if __name__ == '__main__':
    sol = Solution()
    print(sol.intToRoman(3))  # III
    print(sol.intToRoman(4))  # IV
    print(sol.intToRoman(9))  # IX
    print(sol.intToRoman(58))  # LVIII
    print(sol.intToRoman(1994))  # MCMXCIV
