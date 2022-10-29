import { describe, test, expect } from '@jest/globals';
import SegmentTree from './segment_tree';

describe('Segment tree', () => {
    test('Construct correct underlying arr', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SegmentTree(arr);
        expect(st.underlyingArray).toStrictEqual([36, 9, 27, 4, 5, 16, 11, 1, 3, 0, 0, 7, 9, 0, 0]);
    });

    test('Sum 2-4', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SegmentTree(arr);
        expect(st.sum(2, 4)).toBe(21);
    });

    test('Update 2, 4', () => {
        const arr = [1, 3, 5, 7, 9, 11];
        const st = new SegmentTree(arr);
        st.update(2, 8);
        st.update(4, 13);
        expect(st.sum(2, 2)).toBe(8);
        expect(st.sum(4, 4)).toBe(13);
        expect(st.sum(0, 2)).toBe(12);
        expect(st.sum(2, 5)).toBe(39);
    });
});