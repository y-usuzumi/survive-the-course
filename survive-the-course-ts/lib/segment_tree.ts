export class SegmentTree<T, U> {
    private arr: U[];
    private length: number;
    private singletonFunc: (v: T) => U;
    private defaultValueFunc: () => U;
    private aggrFunc: (left: U, right: U) => U;

    constructor(arr: T[], singletonFunc: (v: T) => U, defaultValueFunc: () => U, aggrFunc: (left: U, right: U) => U) {
        const leafLength = Math.pow(2, Math.ceil(Math.log2(arr.length)));
        const totalLength = 2 * leafLength - 1;
        this.arr = new Array(totalLength).fill(0);
        this.length = arr.length;
        this.singletonFunc = singletonFunc;
        this.defaultValueFunc = defaultValueFunc;
        this.aggrFunc = aggrFunc;
        this.populateSegmentTree(arr, 0, 0, arr.length - 1);
    }

    private populateSegmentTree(arr: T[], currIdx: number, left: number, right: number): U {
        if (left === right) {
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
            return this.defaultValueFunc();
        }
        if (currLeft >= fromIdx && currRight <= toIdx) {
            return this.arr[currIdx];
        }
        const mid = this.getMid(currLeft, currRight);
        const leftResult =  this.aggregateHelper(currIdx * 2 + 1, currLeft, mid, fromIdx, toIdx);
        const rightResult = this.aggregateHelper(currIdx * 2 + 2, mid+1, currRight, fromIdx, toIdx);
        return this.aggrFunc(leftResult, rightResult);
    }

    private updateValue(currIdx: number, currLeft: number, currRight: number, idx: number, val: T): U {
        if (currLeft > idx || currRight < idx) {
            return this.arr[currIdx];
        }
        if (currLeft == currRight) {
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