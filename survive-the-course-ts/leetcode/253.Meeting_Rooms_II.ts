function minMeetingRooms(intervals: number[][]): number {
    if (intervals.length <= 1) {
        return intervals.length;
    }

    intervals.sort((a, b) => {
        if (a[0] === b[0]) {
            return a[1] - b[1];
        }
        return a[0] - b[0];
    });

    let result = 0;
    while (intervals.length > 0) {
        intervals = calcOverlaps(intervals);
        result++;
    }
    return result;
};

function calcOverlaps(intervals: number[][]): number[][] {
    if (intervals.length <= 1) {
        return [];
    }
    let result: number[][] = [];
    let left = intervals[0];
    for (let idx = 1; idx < intervals.length; idx++) {
        let right = intervals[idx];
        const overlap = overlaps(left, right);
        if (overlap !== null) {
            result.push(overlap);
        }
        left = left[1] > right[1] ? left : right;
    }
    return result;
}

function overlaps(interval1: number[] | null, interval2: number[] | null): number[] | null {
    if (interval1 === null || interval2 === null) {
        return null;
    }

    if (interval1[0] > interval2[0]) {
        let temp = interval1;
        interval1 = interval2;
        interval2 = temp;
    }
    if (interval2[0] < interval1[1]) {
        return [interval2[0], Math.min(interval1[1], interval2[1])];
    }
    return null;
}

console.log(minMeetingRooms([[5, 8], [6, 8]]));