import { describe, test, expect } from '@jest/globals';
import { MinSegmentTree, SegmentTree, SumSegmentTree } from './segment_tree';

describe('Sum segment tree', () => {
    test('Construct correct underlying arr', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SumSegmentTree(arr);
        expect(st.underlyingArray).toStrictEqual([36, 9, 27, 4, 5, 16, 11, 1, 3, 0, 0, 7, 9, 0, 0]);
    });

    test('Sum 2-4', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SumSegmentTree(arr);
        expect(st.aggregate(2, 4)).toBe(21);
    });

    test('Update 2, 4', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SumSegmentTree(arr);
        st.update(2, 8);
        st.update(4, 13);
        expect(st.aggregate(2, 2)).toBe(8);
        expect(st.aggregate(4, 4)).toBe(13);
        expect(st.aggregate(0, 2)).toBe(12);
        expect(st.aggregate(2, 5)).toBe(39);
    });
});

describe('Min segment tree', () => {
    test('Min of 1-5', () => {
        const arr = [2, 5, 1, 4, 9, 3];
        const st = new MinSegmentTree(arr);
        expect(st.aggregate(1, 5)).toBe(1);
    })
});

describe('Max string segment tree', () => {
    test('Max of 2-4', () => {
        const arr = ["Hello", "World", "Congratulations", "I", "Aggregation"];
        const st = new SegmentTree<string, [number, string]>(arr, v => [v.length, v], () => [0, ''], (left, right) => {
            return left[0] > right[0] ? left : right;
        });
        expect(st.aggregate(2, 4)).toStrictEqual([15, "Congratulations"]);
    });
})