// Problem link: https://leetcode.com/problems/n-queens/

function solveNQueens(n: number): string[][] {
    let results: [number, number][][] = [];
    let stack: [number, number][] = [];
    const blockedRows = new Array(n).fill(false);
    const blockedCols = new Array(n).fill(false);
    const blockedDiagP = new Array(2*n - 1).fill(false);
    const blockedDiagN = new Array(2*n - 1).fill(false);
    solveNQueensHelper(n, 0, n, blockedRows, blockedCols, blockedDiagP, blockedDiagN, stack, results);
    let stringResults = results.map(result => printResult(n, result));
    return stringResults;
};

function printResult(n: number, result: [number, number][]): string[] {
    let stringResult: string[] = [];
    for (const [_, col] of result) {
        stringResult.push('.'.repeat(col) + 'Q' + '.'.repeat(n - col - 1));
    }
    return stringResult;
}

function solveNQueensHelper(
    n: number,
    startRowIdx: number,
    remainingQueens: number,
    blockedRows: boolean[],
    blockedCols: boolean[],
    blockedDiagP: boolean[],
    blockedDiagN: boolean[],
    stack: [number, number][],
    results: [number, number][][]) {
    if (remainingQueens === 0) {
        results.push(stack.slice());
    }
    for (let rowIdx = startRowIdx; rowIdx < n; rowIdx++) {
        if (blockedRows[rowIdx]) {
            continue;
        }

        blockedRows[rowIdx] = true;

        for (let colIdx = 0; colIdx < n; colIdx++) {
            if (blockedCols[colIdx]) {
                continue;
            }
            const diagPIdx = rowIdx - colIdx + n - 1;
            if (blockedDiagP[diagPIdx]) {
                continue;
            }
            const diagNIdx = rowIdx + colIdx;
            if (blockedDiagN[diagNIdx]) {
                continue;
            }

            blockedCols[colIdx] = true;
            blockedDiagP[diagPIdx] = true;
            blockedDiagN[diagNIdx] = true;
            stack.push([rowIdx, colIdx]);
            solveNQueensHelper(n, rowIdx+1, remainingQueens-1, blockedRows, blockedCols, blockedDiagP, blockedDiagN, stack, results);
            stack.pop();
            blockedCols[colIdx] = false;
            blockedDiagP[diagPIdx] = false;
            blockedDiagN[diagNIdx] = false;
        }

        blockedRows[rowIdx] = false;
    }
}

export { };