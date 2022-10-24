// Problem link: https://leetcode.com/problems/kth-largest-element-in-an-array/

function findKthLargest(nums: number[], k: number): number {
    return quickSearch(nums, 0, nums.length - 1, k);
};

function quickSearch(nums: number[], left: number, right: number, k: number) {
    if (left === right) {
        return nums[left];
    }
    let targetIndex = nums.length - k;
    let pivot = nums[right];
    let finalPivotIndex = left;
    for (let idx = left; idx < right; idx++) {
        if (nums[idx] <= pivot) {
            const temp = nums[idx];
            nums[idx] = nums[finalPivotIndex];
            nums[finalPivotIndex] = temp;
            finalPivotIndex++;
        }
    }
    const temp = nums[finalPivotIndex];
    nums[finalPivotIndex] = pivot;
    nums[right] = temp;
    if (finalPivotIndex === targetIndex) {
        return pivot;
    }
    if (finalPivotIndex < targetIndex) {
        return quickSearch(nums, finalPivotIndex+1, right, k);
    }
    return quickSearch(nums, left, finalPivotIndex-1, k);
}

console.log(findKthLargest([3,2,1,5,6,4], 2));

export {};