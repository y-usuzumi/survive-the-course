export type AncestorRankPair = [number, number];

export class DisjointSets {
    private parentMap: number[];

    constructor(size) {
        this.parentMap = new Array(size).fill(-1);
    }

    /**
     * Finds the ancestor and the rank of the set.
     * @param v The element
     * @returns [ancestor, rank]
     */
    findAncestor(v: number): AncestorRankPair {
        while (this.parentMap[v] >= 0) {
            v = this.parentMap[v];
        }
        return [v, -this.parentMap[v]];
    }

    union(v1: number, v2: number): AncestorRankPair {
        const [a1, r1] = this.findAncestor(v1);
        const [a2, r2] = this.findAncestor(v2);
        if (r1 >= r2) {
            this.parentMap[a2] = a1;
            const newRank = r1 === r2 ? r1 + 1 : r1;
            this.parentMap[a1] = -newRank;
            return [a1, newRank];
        } else {
            this.parentMap[a1] = a2;
            return [a2, r2];
        }
    }

    sameSet(v1: number, v2: number): boolean {
        return this.findAncestor(v1)[0] === this.findAncestor(v2)[0];
    }
}