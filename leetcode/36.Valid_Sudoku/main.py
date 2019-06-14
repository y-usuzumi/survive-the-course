from typing import List

class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        appeared_rows = [set() for _ in range(9)]
        appeared_cols = [set() for _ in range(9)]
        appeared_blocks = [set() for _ in range(9)]
        for row_idx, row in enumerate(board):
            test_row = appeared_rows[row_idx]
            for col_idx, cell in enumerate(row):
                if cell == '.':
                    continue
                if cell in test_row:
                    return False
                test_row.add(cell)
                test_col = appeared_cols[col_idx]
                if cell in test_col:
                    return False
                test_col.add(cell)
                test_block = appeared_blocks[3 * (row_idx // 3) + (col_idx // 3)]
                if cell in test_block:
                    return False
                test_block.add(cell)
        return True
