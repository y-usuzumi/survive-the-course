import { UndirectedAdjacencyMatrix, UndirectedGraph, VertexWeightPair } from ".";
import { DisjointSets } from "../disjoint_sets";

export function prims(g: UndirectedGraph): UndirectedGraph {
    const minCostEdges: VertexWeightPair[] = new Array(g.size);
    for (let idx = 0; idx < minCostEdges.length; idx++) {
        minCostEdges[idx] = [-1, Infinity];
    }
    const unvisitedVertices: Set<number> = new Set();
    for (let idx = 0; idx < g.size; idx++) {
        minCostEdges[idx] = [-1, Infinity];
        unvisitedVertices.add(idx);
    }

    let isFirst = true;
    const result = new UndirectedAdjacencyMatrix(g.size);
    while (unvisitedVertices.size > 0) {
        let lowestCostVertex = findLowestCostVertex(minCostEdges, unvisitedVertices);
        if (isFirst) {
            isFirst = false;
        } else {
            let newEdge = minCostEdges[lowestCostVertex];
            result.addEdge(lowestCostVertex, newEdge[0], newEdge[1]);
        }
        unvisitedVertices.delete(lowestCostVertex);
        for (const [v, w] of g.edges(lowestCostVertex)) {
            if (unvisitedVertices.has(v) && w < minCostEdges[v][1]) {
                minCostEdges[v] = [lowestCostVertex, w];
            }
        }
    }
    
    return result;
}

function findLowestCostVertex(vertexCosts: VertexWeightPair[], unvisitedVertices: Set<number>): number {
    let lowestCost = Infinity;
    let lowestCostVertex = -1;
    for (const v of unvisitedVertices) {
        if (vertexCosts[v][1] <= lowestCost) {
            lowestCost = vertexCosts[v][1];
            lowestCostVertex = v;
        }
    }
    return lowestCostVertex;
}

export function kruscals(g: UndirectedGraph): UndirectedGraph {
    const allEdges = g.allEdges().sort(([_v00, _v01, w0], [_v10, _v11, w1]) => w0 - w1);
    const ds = new DisjointSets(g.size);
    const result = new UndirectedAdjacencyMatrix(g.size);
    for (const [v0, v1, w] of allEdges) {
        if (ds.sameSet(v0, v1)) {
            continue;
        }
        result.addEdge(v0, v1, w);
        ds.union(v0, v1);
    }

    return result;
}