// Problem link: https://leetcode.com/problems/valid-sudoku/

function isValidSudoku(board: string[][]): boolean {
    const rowLookup: Map<number, Set<string>> = new Map();
    const colLookup: Map<number, Set<string>> = new Map();
    const squareLookup: Map<number, Set<string>> = new Map();
    for (let rowIdx = 0; rowIdx < board.length; rowIdx++) {
        const row = board[rowIdx];
        for (let colIdx = 0; colIdx < board[0].length; colIdx++) {
            const cell = row[colIdx];
            if (cell === '.') {
                continue;
            }
            if (!rowLookup.has(rowIdx)) {
                rowLookup.set(rowIdx, new Set());
            }
            if (!colLookup.has(colIdx)) {
                colLookup.set(colIdx, new Set());
            }

            let squareIdx = Math.floor(rowIdx / 3) * 3 + Math.floor(colIdx / 3);
            if (!squareLookup.has(squareIdx)) {
                squareLookup.set(squareIdx, new Set());
            }
            if (rowLookup.get(rowIdx)!.has(cell) ||
                colLookup.get(colIdx)!.has(cell) ||
                squareLookup.get(squareIdx)!.has(cell)) {
                return false;
            }
            rowLookup.get(rowIdx)!.add(cell);
            colLookup.get(colIdx)!.add(cell);
            squareLookup.get(squareIdx)!.add(cell);
        }
    }
    return true;
};

export { };