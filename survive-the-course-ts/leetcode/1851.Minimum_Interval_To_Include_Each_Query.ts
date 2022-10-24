import { MinHeap } from '../lib/heap';

function minInterval(intervals: number[][], queries: number[]): number[] {
    intervals.sort((a, b) => {
        if (a[0] === b[0]) {
            return a[1] - b[1];
        }
        return a[0] - b[0];
    });

    const sortedQueries = [...queries].sort((a, b) => a - b);

    let intervalIdx = 0;
    let resultMap = new Map();
    let heap: MinHeap<[number, number]> = new MinHeap([], (a, b) => {
        if (a[0] === b[0]) {
            return a[1] - b[1];
        }
        return a[0] - b[0];
    })

    for (const query of sortedQueries) {
        console.log(`Query: ${query}`);
        while (intervalIdx < intervals.length && intervals[intervalIdx][0] <= query) {
            let [left, right] = intervals[intervalIdx];
            heap.push([right - left + 1, right]);
            intervalIdx++;
        }

        while (heap.size > 0 && heap.peek()![1] < query) {
            const poppedItem = heap.pop();
            console.log(poppedItem);
        }

        resultMap.set(query, heap.size > 0 ? heap.peek()![0] : -1);
    }

    return queries.map(q => resultMap.get(q));
};

console.log(minInterval([[9,9],[1,10],[1,3],[9,10],[8,8]], [1,5,3,10,5]));