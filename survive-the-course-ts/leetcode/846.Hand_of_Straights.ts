// Problem link: https://leetcode.com/problems/hand-of-straights/

import { MinHeap } from "../lib/heap";

function isNStraightHand(hand: number[], groupSize: number): boolean {
    if (hand.length % groupSize !== 0) {
        return false;
    }
    const counter = new Map();
    for (const num of hand) {
        counter.set(num, (counter.get(num) || 0) + 1);
    }

    const keys = Array.from(counter.keys());

    const h = new MinHeap(keys, (a, b) => a - b);

    while (h.size > 0) {
        let start = h.pop();
        let startCount = counter.get(start);
        for (let idx = 1; idx < groupSize; idx++) {
            const count = counter.get(start + idx);
            if (count === undefined || count < startCount) {
                return false;
            }
            counter.set(start + idx, count - startCount);
            if (count - startCount === 0) {
                h.pop();
            }
        }
    }
    return true;
};

export {};