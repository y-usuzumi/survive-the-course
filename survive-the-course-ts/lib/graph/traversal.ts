import { DirectedGraph } from ".";
import { LinkedList } from "../linked_list";

/**
 * Performs depth-first traversal in the directed graph.
 * @param g The graph
 * @param f An action to perform on each vertex. If it returns false, the traversal will stop
 */
export function dfs(g: DirectedGraph, f: (v: number) => boolean) {
    if (g.size === 0) {
        return;
    }
    let visited: Set<number> = new Set();
    for (let v = 0; v < g.size; v++) {
        if (visited.has(v)) {
            continue;
        }
        let stack = [v];
        while (stack.length > 0) {
            const v = stack.pop()!;
            if (visited.has(v)) {
                continue;
            }
            visited.add(v);
            if (!f(v)) {
                return;
            }
            for (const [v2, _w] of g.outEdges(v)) {
                if (visited.has(v2)) {
                    continue;
                }
                stack.push(v2);
            }
        }
    }
}

/**
 * Performs breadth-first traversal in the directed graph
 * @param g The graph
 * @param f An action to perform on each vertex. If it returns false, the traversal will stop
 */
export function bfs(g: DirectedGraph, f: (v: number) => boolean) {
    if (g.size === 0) {
        return;
    }
    const visited: Set<number> = new Set();
    for (let v = 0; v < g.size; v++) {
        if (visited.has(v)) {
            continue;
        }
        const q: LinkedList<number> = new LinkedList();
        q.push(v);
        while (q.size > 0) {
            const v = q.shift()!;
            if (visited.has(v)) {
                continue;
            }
            visited.add(v);
            if (!f(v)) {
                return;
            }
            for (const [v2, _w] of g.outEdges(v)) {
                if (visited.has(v2)) {
                    continue;
                }
                q.push(v2);
            }
        }
    }
}