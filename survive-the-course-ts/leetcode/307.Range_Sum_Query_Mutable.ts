// Problem link: https://leetcode.com/problems/range-sum-query-mutable/description/

class NumArray {
    private arr: number[];
    private length: number;

    constructor(nums: number[]) {
        let leavesLength = 2 ** Math.ceil(Math.log2(nums.length));
        let totalLength = 2 * leavesLength - 1;
        this.arr = new Array(totalLength).fill(0);
        this.length = nums.length;
        this.populate(0, 0, this.length - 1, nums);
    }

    private getMid(left: number, right: number) {
        return left + Math.floor((right - left) / 2);
    }

    private populate(currIdx: number, leftBound: number, rightBound: number, nums: number[]): number {
        if (leftBound === rightBound) {
            this.arr[currIdx] = nums[leftBound];
            return nums[leftBound];
        }

        const mid = this.getMid(leftBound, rightBound);
        const leftResult = this.populate(currIdx * 2 + 1, leftBound, mid, nums);
        const rightResult = this.populate(currIdx * 2 + 2, mid + 1, rightBound, nums);
        const total = leftResult + rightResult;
        this.arr[currIdx] = total;
        return total;
    }

    private updateHelper(currIdx: number, leftBound: number, rightBound: number, targetIdx: number, val: number): number {
        if (targetIdx < leftBound || targetIdx > rightBound) {
            return this.arr[currIdx];
        }
        if (leftBound === rightBound) {
            this.arr[currIdx] = val;
            return val;
        }
        const mid = this.getMid(leftBound, rightBound);
        const leftResult = this.updateHelper(currIdx * 2 + 1, leftBound, mid, targetIdx, val);
        const rightResult = this.updateHelper(currIdx * 2 + 2, mid + 1, rightBound, targetIdx, val);
        const total = leftResult + rightResult;
        this.arr[currIdx] = total;
        return total;
    }

    private sumHelper(currIdx: number, leftBound: number, rightBound: number, leftIdx: number, rightIdx: number): number {
        if (leftBound > rightIdx || rightBound < leftIdx) {
            return 0;
        }

        if (leftBound >= leftIdx && rightBound <= rightIdx) {
            return this.arr[currIdx];
        }

        const mid = this.getMid(leftBound, rightBound);
        const leftResult = this.sumHelper(currIdx * 2 + 1, leftBound, mid, leftIdx, rightIdx);
        const rightResult = this.sumHelper(currIdx * 2 + 2, mid + 1, rightBound, leftIdx, rightIdx);
        const total = leftResult + rightResult;
        return total;
    }

    update(index: number, val: number): void {
        this.updateHelper(0, 0, this.length - 1, index, val);
    }

    sumRange(left: number, right: number): number {
        return this.sumHelper(0, 0, this.length - 1, left, right);
    }
}

class NumArray2 {
    private arr: number[];
    private length: number;

    constructor(nums: number[]) {
        this.arr = new Array(nums.length * 2).fill(0);
        const length = nums.length;
        this.length = length;
        for (let idx = 0; idx < nums.length; idx++) {
            this.arr[idx + length] = nums[idx];
        }
        this.populateTree();
    }

    private populateTree(): void {
        for (let idx = this.length - 1; idx >= 1; idx--) {
            this.arr[idx] = this.arr[idx*2] + this.arr[idx*2 + 1];
        }
    }

    update(index: number, val: number): void {
        index += this.length;
        this.arr[index] = val;

        while (index > 1) {
            this.arr[index >> 1] = this.arr[index] + this.arr[index ^ 1];
            index >>>= 1;
        }
    }

    sumRange(left: number, right: number): number {
        left += this.length;
        right += this.length;
        let result = 0;

        while (left <= right) {
            if (left & 1) {
                result += this.arr[left++];
            }
            if (!(right & 1)) {
                result += this.arr[right--];
            }
            left >>>= 1;
            right >>>= 1;
        }

        return result;
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * var obj = new NumArray(nums)
 * obj.update(index,val)
 * var param_2 = obj.sumRange(left,right)
 */

/**
 * Your NumArray object will be instantiated and called as such:
 * var obj = new NumArray(nums)
 * obj.update(index,val)
 * var param_2 = obj.sumRange(left,right)
 */

const st = new NumArray2([1, 2, 3, 4, 5]);
console.log(st.sumRange(0, 4));

export {};