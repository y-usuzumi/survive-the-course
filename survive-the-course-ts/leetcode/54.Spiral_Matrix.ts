// Problem link: https://leetcode.com/problems/spiral-matrix/

function spiralOrder(matrix: number[][]): number[] {
    return spiralOrderWithOffset(matrix, 0);
}

function spiralOrderWithOffset(matrix: number[][], offset: number): number[] {
    if (matrix.length === 0 || matrix[0].length === 0) {
        return [];
    }
    const rows = matrix.length;
    const cols = matrix[0].length;
    const result: number[] = [];
    let minRowIdx = offset;
    let maxRowIdx = rows - offset - 1;
    if (minRowIdx > maxRowIdx) {
        return [];
    }
    let minColIdx = offset;
    let maxColIdx = cols - offset - 1;
    if (minColIdx > maxColIdx) {
        return [];
    }

    while (minRowIdx <= maxRowIdx && minColIdx <= maxColIdx) {
        for (let colIdx = minColIdx; colIdx <= maxColIdx; colIdx++) {
            result.push(matrix[minRowIdx][colIdx]);
        }
    
        for (let rowIdx = minRowIdx + 1; rowIdx <= maxRowIdx; rowIdx++) {
            result.push(matrix[rowIdx][maxColIdx]);
        }
    
        if (maxRowIdx > minRowIdx) {
            for (let colIdx = maxColIdx - 1; colIdx >= minColIdx; colIdx--) {
                result.push(matrix[maxRowIdx][colIdx]);
            }
        }
    
        if (maxColIdx > minColIdx) {
            for (let rowIdx = maxRowIdx - 1; rowIdx > minRowIdx; rowIdx--) {
                result.push(matrix[rowIdx][minColIdx]);
            }
        }
        minRowIdx++;
        minColIdx++;
        maxRowIdx--;
        maxColIdx--;
    }
    return result;
}

export { };