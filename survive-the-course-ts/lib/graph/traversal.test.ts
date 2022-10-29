import { describe, test, expect } from '@jest/globals';
import { DirectedAdjacencyMatrix } from '.';
import { dfs, bfs } from './traversal';

const eq = (v: number) => (p: number) => p === v;

describe('Graph traversal', () => {
    test('DFS with two SCC', () => {
        const g = new DirectedAdjacencyMatrix(6);
        // SCC 1
        g.addEdge(0, 2, 1);
        g.addEdge(0, 3, 1);
        g.addEdge(2, 3, 1);
        g.addEdge(3, 1, 1);
        // SCC 2
        g.addEdge(4, 5, 1);

        const visitOrder: number[] = [];
        dfs(g, v => {
            visitOrder.push(v);
            return true;
        });
        expect(visitOrder.length).toBe(6);
        expect(visitOrder.findIndex(eq(0))).toBeLessThan(visitOrder.findIndex(eq(2)));
        expect(visitOrder.findIndex(eq(0))).toBeLessThan(visitOrder.findIndex(eq(3)));
        expect(visitOrder.findIndex(eq(3))).toBeLessThan(visitOrder.findIndex(eq(1)));
        expect(visitOrder.findIndex(eq(4))).toBeLessThan(visitOrder.findIndex(eq(5)));
    });

    test('BFS with two SCC', () => {
        const g = new DirectedAdjacencyMatrix(6);
        // SCC 1
        g.addEdge(0, 2, 1);
        g.addEdge(0, 3, 1);
        g.addEdge(2, 3, 1);
        g.addEdge(3, 1, 1);
        // SCC 2
        g.addEdge(4, 5, 1);

        const visitOrder: number[] = [];
        bfs(g, v => {
            visitOrder.push(v);
            return true;
        });
        expect(visitOrder.length).toBe(6);
        expect(visitOrder.findIndex(eq(0))).toBeLessThan(visitOrder.findIndex(eq(2)));
        expect(visitOrder.findIndex(eq(0))).toBeLessThan(visitOrder.findIndex(eq(3)));
        expect(visitOrder.findIndex(eq(2))).toBeLessThan(visitOrder.findIndex(eq(1)));
        expect(visitOrder.findIndex(eq(3))).toBeLessThan(visitOrder.findIndex(eq(1)));
        expect(visitOrder.findIndex(eq(4))).toBeLessThan(visitOrder.findIndex(eq(5)));
    })
});