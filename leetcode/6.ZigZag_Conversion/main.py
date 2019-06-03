# 1       9       h
#  2     8 a     g
#   3   7   b   f
#    4 6     c e
#     5       d
from io import StringIO
class Solution:
    def convert(self, s:str, num_rows: int) -> str:
        if num_rows == 1:
            return s
        chars = [None] * len(s)
        loop_size = 1 if num_rows == 1 else (num_rows - 1) * 2
        loop_cnt, loop_rem = len(s) // loop_size, len(s) % loop_size
        start_point = 0
        for loop_idx in range(0, loop_cnt):
            chars[loop_idx] = s[loop_size * loop_idx]
        start_point += loop_cnt
        if loop_rem > 0:
            chars[loop_cnt] = s[loop_size * loop_cnt]
            start_point += 1
        for row_idx in range(1, num_rows-1):
            for loop_idx in range(0, loop_cnt):
                chars[start_point + 2 * loop_idx] = s[loop_size * loop_idx + row_idx]
                chars[start_point + 2 * loop_idx + 1] = s[loop_size * loop_idx + loop_size - row_idx]
            start_point += 2 * loop_cnt
            if loop_rem > row_idx:
                chars[start_point] = s[loop_size * loop_cnt + row_idx]
                start_point += 1
                if loop_rem >= 2 * num_rows - row_idx - 1:
                    chars[start_point] = s[loop_size * loop_cnt + 2 * num_rows - row_idx - 2]
                    start_point += 1
        for loop_idx in range(0, loop_cnt):
            chars[start_point + loop_idx] = s[loop_size * loop_idx + num_rows - 1]
        start_point += loop_cnt
        if loop_rem >= num_rows:
            chars[start_point] = s[loop_size * loop_cnt + num_rows - 1]

        return ''.join(chars)

    # 下面这个算法刚好弄反了，按行给定输入，输出zigzag后的字符串。
    def _convert(self, s: str, numRows: int) -> str:
        if not s:
            return s
        num_rows = numRows
        chars = [None] * len(s)
        loop_size = 1 if num_rows == 1 else (num_rows-1) * 2
        loop_cnt, loop_rem = len(s) // loop_size, len(s) % loop_size
        fst_row_size = loop_cnt + (1 if loop_rem else 0)
        for idx, ch in enumerate(s):
            if idx >= fst_row_size:
                break
            chars[idx * loop_size] = ch
        for row_idx in range(1, num_rows-1):
            mid_row_size = loop_cnt * 2
            start_point = fst_row_size + mid_row_size * (row_idx - 1)
            if loop_rem >= 2 * num_rows - row_idx - 1:
                mid_row_size += 2
            elif loop_rem > row_idx:
                mid_row_size += 1
            if mid_row_size % 2 == 0:
                for idx in range(0, mid_row_size, 2):
                    loop_idx = idx // 2
                    chars[loop_idx * loop_size + row_idx] = s[start_point + idx]
                    chars[loop_idx * loop_size + 2 * num_rows - row_idx - 2] = s[start_point + idx + 1]
            else:
                for idx in range(0, mid_row_size - 1, 2):
                    loop_idx = idx // 2
                    chars[loop_idx * loop_size + row_idx] = s[start_point + idx]
                    chars[loop_idx * loop_size + 2 * num_rows - row_idx - 2] = s[start_point + idx + 1]
                loop_idx = idx // 2
                chars[loop_idx * loop_size + row_idx] = s[start_point + idx]
        if num_rows > 1:
            start_point = fst_row_size + mid_row_size * row_idx
            for idx in range(0, len(s) - start_point):
                chars[idx * loop_size + num_rows - 1] = s[start_point + idx]

        print(chars)
        return ''.join(chars)


if __name__ == '__main__':
    sol = Solution()
    # s = "19h28ag37bf46ce5d"
    # print(sol._convert(s, 5))
    # s = "PAYPALISHIRING"
    # print(sol.convert(s, 3))

    # s = "PAYPALISHIRING"
    # print(sol.convert(s, 4))

    # s = "ABCDEFGHIJKL"
    # print(sol.convert(s, 3))

    s = "ABC"
    print(sol.convert(s, 3))
