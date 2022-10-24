import { describe, test, expect } from '@jest/globals';
import { DisjointSets, AncestorRankPair } from '.';

describe('Disjoint sets', () => {
    test('findAncestor returns itself', () => {
        const ds = new DisjointSets(4);
        expect(ds.findAncestor(0)).toStrictEqual([0, 1]);
    });

    test('Elements have same ancestor after union', () => {
        const ds = new DisjointSets(4);
        ds.union(0, 1);
        ds.union(2, 3);
        const [a1, _r1] = ds.findAncestor(1);
        const [a2, _r2] = ds.findAncestor(2);
        expect(a1).not.toBe(a2);
        ds.union(1, 2);
        expect(ds.findAncestor(1)).toStrictEqual(ds.findAncestor(3));
    });

    test('Correct rank after uniting sets with different ranks', () => {
        const ds = new DisjointSets(4);
        ds.union(2, 3);
        const [_a1, r1] = ds.findAncestor(1);
        const [_a2, r2] = ds.findAncestor(2);
        expect(r1).toBe(1);
        expect(r2).toBe(2);
        ds.union(1, 2);
        expect(ds.findAncestor(1)[1]).toBe(2);
    });

    test('Correct rank after uniting sets with the same rank', () => {
        const ds = new DisjointSets(4);
        ds.union(0, 1);
        ds.union(2, 3);
        const [_a1, r1] = ds.findAncestor(1);
        const [_a2, r2] = ds.findAncestor(2);
        expect(r1).toBe(2);
        expect(r2).toBe(2);
        ds.union(1, 2);
        expect(ds.findAncestor(1)[1]).toBe(3);
    });
});