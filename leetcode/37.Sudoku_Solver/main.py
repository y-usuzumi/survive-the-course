from typing import List

all_nums = {"1", "2", "3", "4", "5", "6", "7", "8", "9"}

class Solution:
    def _find_block(self, row, col):
        return 3 * (row // 3) + col // 3

    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        appeared_rows = [set() for _ in range(9)]
        appeared_cols = [set() for _ in range(9)]
        appeared_blocks = [set() for _ in range(9)]
        varying_cells = []
        for row_idx, row in enumerate(board):
            for col_idx, cell in enumerate(row):
                if cell != '.':
                    appeared_rows[row_idx].add(cell)
                    appeared_cols[col_idx].add(cell)
                    appeared_blocks[self._find_block(row_idx, col_idx)].add(cell)
                else:
                    varying_cells.append((row_idx, col_idx))
        if not varying_cells:
            return
        vals = self._solve_sudoku(appeared_rows, appeared_cols, appeared_blocks, varying_cells, 0, [])
        if vals:
            for (row_idx, col_idx), val in zip(varying_cells, vals):
                board[row_idx][col_idx] = val

    def _solve_sudoku(self, appeared_rows, appeared_cols, appeared_blocks, varying_cells, varying_cells_idx, vals):
        if varying_cells_idx >= len(varying_cells):
            return vals
        row_idx, col_idx = varying_cells[varying_cells_idx]
        appeared_row = appeared_rows[row_idx]
        appeared_col = appeared_cols[col_idx]
        appeared_block = appeared_blocks[self._find_block(row_idx, col_idx)]
        rem_nums = all_nums - appeared_row - appeared_col - appeared_block
        if not rem_nums:
            return None
        for num in rem_nums:
            appeared_row.add(num)
            appeared_col.add(num)
            appeared_block.add(num)
            vals.append(num)
            result = self._solve_sudoku(appeared_rows, appeared_cols, appeared_blocks, varying_cells, varying_cells_idx+1, vals)
            if result:
                return result
            appeared_row.remove(num)
            appeared_col.remove(num)
            appeared_block.remove(num)
            vals.pop()
