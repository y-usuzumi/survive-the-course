type SortFunc<T> = (a: T, b: T) => number;

export class MinHeap<T> {
    private vals: T[];
    private sortFunc: SortFunc<T>;

    constructor(vals: T[] = [], sortFunc: SortFunc<T>) {
        this.vals = [null, ...vals];
        if (sortFunc === undefined) {
            throw new Error("You need to provide a sortFunc");
        } else {
            this.sortFunc = sortFunc;
        }
        if (this.vals.length > 1) {
            this.buildHeap();
        }
    }

    isLT(left: T, right: T): boolean {
        return this.sortFunc(left, right) < 0;
    }

    isLE(left: T, right: T): boolean {
        return this.sortFunc(left, right) <= 0;
    }

    isGT(left: T, right: T): boolean {
        return !this.isLE(left, right);
    }

    isGE(left: T, right: T): boolean {
        return !this.isLT(left, right);
    }

    private get siftDownStartIdx() {
        return Math.floor((this.vals.length - 1) / 2);
    }

    private buildHeap() {
        let currIdx = this.siftDownStartIdx;
        while (currIdx > 0) {
            this.siftDown(currIdx);
            currIdx--;
        }
    }

    private siftUp(currIdx: number) {
        while (currIdx > 1) {
            const currVal = this.vals[currIdx];
            const parentIdx = Math.floor(currIdx / 2);
            const parentVal = this.vals[parentIdx];
            if (this.isGE(currVal, parentVal)) {
                break;
            }
            const temp = parentVal;
            this.vals[parentIdx] = currVal;
            this.vals[currIdx] = temp;
            currIdx = parentIdx;
        }
    }

    private siftDown(currIdx: number) {
        const terminateIdx = this.siftDownStartIdx;
        while (currIdx <= terminateIdx) {
            let leftChildIdx = currIdx * 2;
            let rightChildIdx = currIdx * 2 + 1;
            if (rightChildIdx < this.vals.length) {
                let currVal = this.vals[currIdx];
                let leftChild = this.vals[leftChildIdx];
                let rightChild = this.vals[rightChildIdx];
                if (this.isGT(currVal, leftChild) || this.isGT(currVal, rightChild)) {
                    let smallerIdx = this.isLT(leftChild, rightChild) ? leftChildIdx : rightChildIdx;
                    let temp = this.vals[smallerIdx];
                    this.vals[smallerIdx] = currVal;
                    this.vals[currIdx] = temp;
                    currIdx = smallerIdx;
                } else {
                    break; // No more siftDown needed;
                }

            } else {
                let currVal = this.vals[currIdx];
                let leftChild = this.vals[leftChildIdx];
                if (this.isGT(currVal, leftChild)) {
                    this.vals[currIdx] = leftChild;
                    this.vals[leftChildIdx] = currVal;
                    currIdx = leftChildIdx;
                } else {
                    break; // No more heapify siftDown needed;
                }
            }
        }
    }

    push(...vals: T[]) {
        for (const val of vals) {
            this.vals.push(val);
            this.siftUp(this.vals.length - 1);
        }
    }

    pop() {
        if (this.vals.length === 1) {
            return undefined;
        }
        const result = this.vals[1];
        this.vals[1] = this.vals[this.vals.length - 1];
        this.vals.length -= 1;
        this.siftDown(1);
        return result;
    }

    peek() {
        if (this.vals.length === 1) {
            return undefined;
        }
        return this.vals[1];
    }

    get size() {
        return this.vals.length - 1;
    }
}

function test() {
    let h = new MinHeap([1, 17, 9, 7, 2, 8, 5, 3, 10, 12], (a, b) => a - b);
    h.push(4, 10, 11, 6);
    console.log(h);
    while (h.size > 0) {
        console.log(h.pop());
    }
}