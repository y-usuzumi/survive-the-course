function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
    if (nums1.length > nums2.length) {
        const temp = nums1;
        nums1 = nums2;
        nums2 = temp;
    }
    const total = nums1.length + nums2.length;
    const leftSize = Math.floor(total / 2);
    let l = 0;
    let r = nums1.length - 1;
    while (true) {
        const mid1 = Math.floor((l + r) / 2);
        const mid2 = leftSize - mid1 - 2;
        let l1 = nums1[mid1] !== undefined ? nums1[mid1] : -Infinity;
        let r1 = nums1[mid1+1] !== undefined ? nums1[mid1+1] : Infinity;
        let l2 = nums2[mid2] !== undefined ? nums2[mid2] : -Infinity;
        let r2 = nums2[mid2+1] !== undefined ? nums2[mid2+1]: Infinity;
        if (l1 <= r2 && l2 <= r1) {
            if (total % 2 === 0) {
                const leftMax = Math.max(l1, l2);
                const rightMin = Math.min(r1, r2);
                return (leftMax + rightMin) / 2;
            } else {
                return Math.min(r1, r2);
            }
        } else if (l1 > r2) {
            r = mid1 - 1;
        } else {
            l = mid1 + 1;
        }
    }
};

// console.log(findMedianSortedArrays([1, 2, 3, 4, 5, 6], [7, 8, 9, 10]))
console.log(findMedianSortedArrays([0, 0], [0, 0]))

export {};