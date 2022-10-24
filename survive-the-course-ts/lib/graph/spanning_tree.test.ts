import { describe, test, expect } from '@jest/globals';
import { UndirectedAdjacencyList } from '.';

import { kruscals, prims } from './spanning_tree';
import { SimpleDirectedGraphVisualizer, SimpleUndirectedGraphVisualizer } from './visualizer';

describe('Minimum spanning tree', () => {
    test('Prim\'s algorithm', () => {
        const g = new UndirectedAdjacencyList(6);
        g.addEdge(0, 1, 4);
        g.addEdge(0, 2, 4);
        g.addEdge(1, 2, 2);
        g.addEdge(2, 3, 3);
        g.addEdge(2, 4, 2);
        g.addEdge(3, 5, 3);
        g.addEdge(4, 5, 3);
        g.addEdge(2, 5, 4);
        const result = prims(g);
        // TODO: verify prims result
        console.log(new SimpleUndirectedGraphVisualizer().visualize(result));
    });

    test('Kruskal\'s algorithm', () => {
        const g = new UndirectedAdjacencyList(6);
        g.addEdge(0, 1, 4);
        g.addEdge(0, 2, 4);
        g.addEdge(1, 2, 2);
        g.addEdge(2, 3, 3);
        g.addEdge(2, 4, 2);
        g.addEdge(3, 5, 3);
        g.addEdge(4, 5, 3);
        g.addEdge(2, 5, 4);
        const result = kruscals(g);
        // TODO: verify kruskals result
        console.log(new SimpleUndirectedGraphVisualizer().visualize(result));
    });
})