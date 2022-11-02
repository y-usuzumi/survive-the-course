// Problem link: https://leetcode.com/problems/merge-triplets-to-form-target-triplet/

function mergeTriplets(triplets: number[][], target: number[]): boolean {
    const fulfilled = new Array(target.length).fill(false);
    let unfulfilled = target.length;
    for (const triplet of triplets) {
        let currFulfilled: number[] = [];
        let skip = false;
        for (let idx = 0; idx < triplet.length; idx++) {
            if (triplet[idx] > target[idx]) {
                skip = true;
                break;
            }
            if (triplet[idx] === target[idx] && !fulfilled[idx]) {
                currFulfilled.push(idx);
            }
        }

        if (skip) {
            continue;
        }

        for (const fulfilledIdx of currFulfilled) {
            fulfilled[fulfilledIdx] = true;
        }
        unfulfilled -= currFulfilled.length;

        if (unfulfilled === 0) {
            return true;
        }
    }
    return false;
};

export {};