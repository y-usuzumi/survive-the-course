/**
 Do not return anything, modify matrix in-place instead.
 */
function rotate(matrix: number[][]): void {
    const length = matrix.length;
    for (let rowIdx = 0; rowIdx < Math.floor(length / 2) + 1; rowIdx++) {
        for (let colIdx = rowIdx; colIdx < length - rowIdx - 1; colIdx++) {
            let temp = matrix[rowIdx][colIdx];
            matrix[rowIdx][colIdx] = matrix[length - colIdx - 1][rowIdx];
            matrix[length - colIdx - 1][rowIdx] = matrix[length - rowIdx - 1][length - colIdx - 1];
            matrix[length - rowIdx - 1][length - colIdx - 1] = matrix[colIdx][length - rowIdx - 1];
            matrix[colIdx][length - rowIdx - 1] = temp;
        }
    }
};