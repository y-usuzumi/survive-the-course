function eraseOverlapIntervals(intervals: number[][]): number {
    if (intervals.length <= 1) {
        return 0;
    }

    intervals.sort((a, b) => {
        if (a[0] !== b[0]) {
            return a[0] - b[0];
        }
        return a[1] - b[1];
    });

    let result = 0;
    let idxl = 0;
    let idxr = 1;
    while (idxr < intervals.length) {
        let intervalL = intervals[idxl];
        let intervalR = intervals[idxr];
        if (intervalR[0] >= intervalL[1]) {
            idxl = idxr;
            idxr++;
        } else if (intervalL[1] >= intervalR[1]) {
            result++;
            idxl = idxr;
            idxr++;
        } else {
            result++;
            idxr++;
        }
    }
    return result;
};

export {};