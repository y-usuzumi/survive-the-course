// Problem link: https://leetcode.com/problems/house-robber/description/

function rob(nums: number[]): number {
    if (nums.length === 0) {
        return 0;
    }
    if (nums.length === 1) {
        return nums[0];
    }

    let maxPrevPrev = nums[0];
    let maxPrev = Math.max(maxPrevPrev, nums[1]);
    for (let idx = 2; idx < nums.length; idx++) {
        const currMax = Math.max(maxPrev, maxPrevPrev + nums[idx]);
        maxPrevPrev = maxPrev;
        maxPrev = currMax;
    }
    return maxPrev;
};

export {};