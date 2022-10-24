class MaxHeap {
    private vals: (number | null)[];

    constructor() {
        this.vals = [null];
    }

    get isEmpty(): boolean {
        return this.vals.length <= 1;
    }

    get size(): number {
        return this.vals.length - 1;
    }

    push(val: number) {
        this.vals.push(val);
        let parentIdx = Math.floor((this.vals.length - 1) / 2);
        while (parentIdx >= 1) {
            const leftIdx = parentIdx * 2;
            const rightIdx = parentIdx * 2 + 1;
            let maxIdx = leftIdx;
            if (rightIdx < this.vals.length) {
                maxIdx = this.vals[leftIdx]! < this.vals[rightIdx]! ? rightIdx : leftIdx;
            };
            if (this.vals[parentIdx]! < this.vals[maxIdx]!) {
                const temp = this.vals[parentIdx];
                this.vals[parentIdx] = this.vals[maxIdx];
                this.vals[maxIdx] = temp;
                parentIdx = Math.floor(parentIdx / 2);
            } else {
                break;
            }
        }
    }

    pop(): number | null {
        if (this.isEmpty) {
            return null;
        }
        const result = this.vals[1];
        this.vals[1] = this.vals[this.vals.length - 1];
        this.vals.splice(this.vals.length - 1);
        let parentIdx = 1;
        while (parentIdx * 2 <= this.vals.length - 1) {
            const leftIdx = parentIdx * 2;
            const rightIdx = parentIdx * 2 + 1;
            let maxIdx = leftIdx;
            if (rightIdx < this.vals.length) {
                maxIdx = this.vals[leftIdx]! < this.vals[rightIdx]! ? rightIdx : leftIdx;
            }

            if (this.vals[parentIdx]! < this.vals[maxIdx]!) {
                const temp = this.vals[parentIdx];
                this.vals[parentIdx] = this.vals[maxIdx];
                this.vals[maxIdx] = temp;
                parentIdx = maxIdx;
            } else {
                break;
            }
        }
        return result;
    }
}

function lastStoneWeight(stones: number[]): number {
    const h = new MaxHeap();
    for (const stone of stones) {
        h.push(stone);
    }

    while (h.size >= 2) {
        const a = h.pop();
        const b = h.pop();
        h.push(Math.abs(a! - b!));
    }

    return h.isEmpty ? 0 : h.pop()??-1;
};


// const h = new MaxHeap();
// for (const val of [3, 5, 18, 9, 2, 10, 4, 3, 7, 1]) {
//     h.push(val);
// }

// const sorted = [];
// while (!h.isEmpty) {
//     sorted.push(h.pop());
// }
// console.log(sorted);
