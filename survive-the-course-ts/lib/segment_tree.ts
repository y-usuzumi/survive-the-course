/**
 * Simple segment tree implementation.
 * @template T: Type of element in the original array
 * @template U: Type of the aggregated result
 */
export class SegmentTree<T, U> {
    private arr: U[];
    private length: number;
    private singletonFunc: (v: T) => U;
    private defaultValueFunc: () => U;
    private aggrFunc: (left: U, right: U) => U;

    /**
     * Constructs a segment tree from an array.
     * @param arr Array from which a segment tree is constructed
     * @param singletonFunc Function that converts a single value to an aggregated value
     * @param defaultValueFunc Function that returns a default value when the current range is out of the query range
     * @param aggrFunc Aggregation function
     */
    constructor(arr: T[], singletonFunc: (v: T) => U, defaultValueFunc: () => U, aggrFunc: (left: U, right: U) => U) {
        // Total number of leaves in the segment tree (including those that may not be occupied)
        const leafLength = Math.pow(2, Math.ceil(Math.log2(arr.length)));
        // Total number of nodes in the segment tree
        const totalLength = 2 * leafLength - 1;
        this.arr = new Array(totalLength);
        for (let idx = 0; idx < this.arr.length; idx++) {
            this.arr[idx] = defaultValueFunc();
        }
        this.length = arr.length;
        this.singletonFunc = singletonFunc;
        this.defaultValueFunc = defaultValueFunc;
        this.aggrFunc = aggrFunc;
        this.populateSegmentTree(arr, 0, 0, arr.length - 1);
    }

    private populateSegmentTree(arr: T[], currIdx: number, left: number, right: number): U {
        if (left === right) {
            // Range contains a single value. Therefore we use the singletonFunc to create an initial aggregated value
            const singletonResult = this.singletonFunc(arr[left]);
            this.arr[currIdx] = singletonResult;
            return singletonResult;
        }
        let mid = this.getMid(left, right);
        let leftResult = this.populateSegmentTree(arr, currIdx * 2 + 1, left, mid);
        let rightResult = this.populateSegmentTree(arr, currIdx * 2 + 2, mid+1, right);
        let aggrResult = this.aggrFunc(leftResult, rightResult);
        this.arr[currIdx] = aggrResult;
        return aggrResult;
    }

    private getMid(left: number, right: number) {
        return Math.floor((left + right) / 2);
    }

    private aggregateHelper(currIdx: number, currLeft: number, currRight: number, fromIdx: number, toIdx: number): U {
        if (currLeft > toIdx || currRight < fromIdx) {
            // Current range is out of the query range. We return a default value for aggregation
            return this.defaultValueFunc();
        }
        if (currLeft >= fromIdx && currRight <= toIdx) {
            // Current range completely lies within the query range. Return the stored aggregation value
            return this.arr[currIdx];
        }
        const mid = this.getMid(currLeft, currRight);
        const leftResult =  this.aggregateHelper(currIdx * 2 + 1, currLeft, mid, fromIdx, toIdx);
        const rightResult = this.aggregateHelper(currIdx * 2 + 2, mid+1, currRight, fromIdx, toIdx);
        return this.aggrFunc(leftResult, rightResult);
    }

    /**
     * Updates the segment tree reflectively to the new value in the original array.
     * @param currIdx Index of the current node (which is to be updated)
     * @param currLeft Left bound of the range that the current node covers
     * @param currRight Right bound of the range that the current node covers
     * @param idx Index of the value in the original array to update
     * @param val New value
     * @returns Updated aggregated value
     */
    private updateValue(currIdx: number, currLeft: number, currRight: number, idx: number, val: T): U {
        if (currLeft > idx || currRight < idx) {
            return this.arr[currIdx];
        }
        if (currLeft === currRight) {
            const singletonResult = this.singletonFunc(val)
            this.arr[currIdx] = singletonResult;
            return singletonResult;
        }
        const mid = this.getMid(currLeft, currRight);
        const leftResult = this.updateValue(currIdx * 2 + 1, currLeft, mid, idx, val);
        const rightResult = this.updateValue(currIdx * 2 + 2, mid+1, currRight, idx, val);
        const aggrResult = this.aggrFunc(leftResult, rightResult);
        this.arr[currIdx] = aggrResult;
        return aggrResult;
    }

    get size(): number {
        return this.length;
    }

    get underlyingArray(): U[] {
        return this.arr;
    }

    aggregate(fromIndex: number, toIndex: number): U {
        return this.aggregateHelper(0, 0, this.length - 1, fromIndex, toIndex);
    }

    update(index: number, value: T) {
        this.updateValue(0, 0, this.length - 1, index, value);
    }
}

export class SumSegmentTree extends SegmentTree<number, number> {
    constructor(arr: number[]) {
        super(arr, a => a, () => 0, (left, right) => left + right);
    }
}

export class MinSegmentTree extends SegmentTree<number, number> {
    constructor(arr: number[]) {
        super(arr, a => a, () => Infinity, (left, right) => Math.min(left, right));
    }
}