function canAttendMeetings(intervals: number[][]): boolean {
    if (intervals.length <= 1) {
        return true;
    }

    intervals.sort((a, b) => {
        if (a[0] === b[0]) {
            return a[1] - b[1];
        }
        return a[0] - b[0];
    });

    let i = 0;
    while (i < intervals.length - 1) {
        if (intervals[i+1][0] < intervals[i][1]) {
            return false;
        }
        i++;
    }

    return true;
};