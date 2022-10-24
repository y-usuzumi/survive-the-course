import { DirectedGraph, UndirectedGraph } from ".";

export interface Visualizer<G, O> {
    visualize(g: G): O;
}

export class SimpleDirectedGraphVisualizer implements Visualizer<DirectedGraph, string> {
    visualize(g: DirectedGraph): string {
        let output = "";
        for (let idx = 0; idx < g.size; idx++) {
            output += `${idx}:\n`;
            for (const [v, w] of g.outEdges(idx)) {
                output += `  -> ${v} (${w})\n`;
            }
        }
        return output;
    }
}

export class SimpleUndirectedGraphVisualizer implements Visualizer<UndirectedGraph, string> {
    visualize(g: UndirectedGraph): string {
        let output = `Size: ${g.size}\n`;
        for (const [v1, v2, w] of g.allEdges()) {
            output += `  ${v1} <-${w}-> ${v2}\n`;
        }
        return output;
    }
}