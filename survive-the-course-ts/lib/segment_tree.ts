export default class SegmentTree {
    private arr: number[];
    private length: number;

    constructor(arr: number[]) {
        const leafLength = Math.pow(2, Math.ceil(Math.log2(arr.length)));
        const totalLength = 2 * leafLength - 1;
        this.arr = new Array(totalLength).fill(0);
        this.length = arr.length;
        this.populateSegmentTree(arr, 0, 0, arr.length - 1);
    }

    private populateSegmentTree(arr: number[], currIdx: number, left: number, right: number): number {
        if (left === right) {
            this.arr[currIdx] = arr[left];
            return arr[left];
        }
        let mid = this.getMid(left, right);
        let leftSum = this.populateSegmentTree(arr, currIdx * 2 + 1, left, mid);
        let rightSum = this.populateSegmentTree(arr, currIdx * 2 + 2, mid+1, right);
        let total = leftSum + rightSum;
        this.arr[currIdx] = total;
        return total;
    }

    private getMid(left: number, right: number) {
        return Math.floor((left + right) / 2);
    }

    private getSum(currIdx: number, currLeft: number, currRight: number, fromIdx: number, toIdx: number) {
        if (currLeft > toIdx || currRight < fromIdx) {
            return 0;
        }
        if (currLeft >= fromIdx && currRight <= toIdx) {
            return this.arr[currIdx];
        }
        const mid = this.getMid(currLeft, currRight);
        const leftSum =  this.getSum(currIdx * 2 + 1, currLeft, mid, fromIdx, toIdx);
        const rightSum = this.getSum(currIdx * 2 + 2, mid+1, currRight, fromIdx, toIdx);
        return leftSum + rightSum;
    }

    private updateValue(currIdx: number, currLeft: number, currRight: number, idx: number, val: number): number {
        if (currLeft > idx || currRight < idx) {
            return this.arr[currIdx];
        }
        if (currLeft == currRight) {
            this.arr[currIdx] = val;
            return val;
        }
        const mid = this.getMid(currLeft, currRight);
        const leftResult = this.updateValue(currIdx * 2 + 1, currLeft, mid, idx, val);
        const rightResult = this.updateValue(currIdx * 2 + 2, mid+1, currRight, idx, val);
        const newSum = leftResult + rightResult;
        this.arr[currIdx] = newSum;
        return newSum;
    }

    get size(): number {
        return this.length;
    }

    get underlyingArray(): number[] {
        return this.arr;
    }

    sum(fromIndex: number, toIndex: number): number {
        return this.getSum(0, 0, this.length - 1, fromIndex, toIndex);
    }

    update(index: number, value: number) {
        this.updateValue(0, 0, this.length - 1, index, value);
    }
}