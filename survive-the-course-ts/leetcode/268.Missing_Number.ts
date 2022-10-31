// Problem link: https://leetcode.com/problems/missing-number/

function missingNumber(nums: number[]): number {
    let completeBits = 0;
    
    for (let idx = 0; idx <= nums.length; idx++) {
        completeBits ^= idx;
    }

    for (const num of nums) {
        completeBits ^= num;
    }

    return completeBits;
};

export {};