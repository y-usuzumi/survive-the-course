import { LinkedList } from "../linked_list";

export type VertexWeightPair = [number, number];
export type EdgeWeightTriplet = [number, number, number];

export interface DirectedGraph {
    get size(): number;
    outEdges(v: number): VertexWeightPair[];
    inEdges(v: number): VertexWeightPair[];
    allEdges(): EdgeWeightTriplet[];
}

export interface UndirectedGraph {
    get size(): number;
    edges(v: number): VertexWeightPair[];
    allEdges(): EdgeWeightTriplet[];
}

export class DirectedAdjacencyMatrix implements DirectedGraph {
    protected matrix: number[][];

    constructor(size: number) {
        this.matrix = new Array(size);
        for (let idx = 0; idx < size; idx++) {
            this.matrix[idx] = new Array(size).fill(0);
        }
    }

    get size() {
        return this.matrix.length;
    }

    addEdge(v1: number, v2: number, w: number) {
        this.matrix[v1][v2] = w;
    }

    removeEdge(v1: number, v2: number) {
        this.matrix[v1][v2] = 0;
    }

    outEdges(v: number): VertexWeightPair[] {
        let result: VertexWeightPair[] = [];
        for (let idx = 0; idx < this.matrix.length; idx++) {
            let weight = this.matrix[v][idx];
            if (weight > 0) {
                result.push([idx, weight]);
            }
        }
        return result;
    }

    inEdges(v: number): VertexWeightPair[] {
        let result: VertexWeightPair[] = [];
        for (let idx = 0; idx < this.matrix.length; idx++) {
            let weight = this.matrix[idx][v];
            if (weight > 0) {
                result.push([idx, weight]);
            }
        }
        return result;
    }

    allEdges(): EdgeWeightTriplet[] {
        let result: EdgeWeightTriplet[] = [];
        for (let v0 = 0; v0 < this.matrix.length; v0++) {
            for (const [v1, w] of this.outEdges(v0)) {
                result.push([v0, v1, w]);
            }
        }
        return result;
    }
}

export class UndirectedAdjacencyMatrix extends DirectedAdjacencyMatrix implements DirectedGraph, UndirectedGraph {
    addEdge(v1: number, v2: number, w: number): void {
        super.addEdge(v1, v2, w);
        super.addEdge(v2, v1, w);
    }

    removeEdge(v1: number, v2: number): void {
        super.removeEdge(v1, v2);
        super.removeEdge(v2, v1);
    }

    edges(v: number): VertexWeightPair[] {
        return this.outEdges(v);
    }

    allEdges(): EdgeWeightTriplet[] {
        let result: EdgeWeightTriplet[] = [];
        for (let v0 = 0; v0 < this.matrix.length; v0++) {
            for (let v1 = v0+1; v1 < this.matrix.length; v1++) {
                const w = this.matrix[v0][v1];
                if (w > 0) {
                    result.push([v0, v1, w]);
                }
            }
        }
        return result;
    }
}

export class DirectedAdjacencyList implements DirectedGraph {
    adjList: LinkedList<VertexWeightPair>[];

    constructor(size: number) {
        this.adjList = new Array(size);
        for (let idx = 0; idx < size; idx++) {
            this.adjList[idx] = new LinkedList();
        }
    }

    get size() {
        return this.adjList.length;
    }

    addEdge(v1: number, v2: number, w: number) {
        this.adjList[v1].append([v2, w]);
    }

    removeEdge(v1: number, v2: number) {
        this.adjList[v1].remove(v => v[0] === v2);
    }

    outEdges(v: number): VertexWeightPair[] {
        return this.adjList[v].toArray();
    }

    inEdges(v: number): VertexWeightPair[] {
        let result: VertexWeightPair[] = [];
        for (let idx = 0; idx < this.adjList.length; idx++) {
            let val = this.adjList[idx].find(([inV, _inW]) => inV === v);
            if (val !== null) {
                result.push(val);
            }
        }
        return result;
    }

    allEdges(): EdgeWeightTriplet[] {
        let result: EdgeWeightTriplet[] = [];
        for (let v0 = 0; v0 < this.adjList.length; v0++) {
            for (const [v1, w] of this.outEdges(v0)) {
                result.push([v0, v1, w]);
            }
        }
        return result;
    }
}

export class UndirectedAdjacencyList extends DirectedAdjacencyList implements DirectedGraph, UndirectedGraph {
    addEdge(v1: number, v2: number, w: number): void {
        super.addEdge(v1, v2, w);
        super.addEdge(v2, v1, w);
    }

    removeEdge(v1: number, v2: number): void {
        super.removeEdge(v1, v2);
        super.removeEdge(v2, v1);
    }

    edges(v: number): VertexWeightPair[] {
        return this.outEdges(v);
    }

    allEdges(): EdgeWeightTriplet[] {
        let result: EdgeWeightTriplet[] = [];
        for (let v0 = 0; v0 < this.adjList.length; v0++) {
            for (const [v1, w] of this.outEdges(v0)) {
                if (v1 > v0) {
                    result.push([v0, v1, w]);
                }
                
            }
        }
        return result;
    }
}