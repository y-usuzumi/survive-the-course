// Problem link: https://leetcode.com/problems/3sum/

function threeSum(nums: number[]): number[][] {
    nums.sort((a, b) => a - b);
    const counter: Map<number, number> = new Map();
    for (const num of nums) {
        if (!counter.has(num)) {
            counter.set(num, 1);
        } else {
            counter.set(num, counter.get(num)! + 1);
        }

    }
    const result: number[][] = [];
    for (let i = 0; i < nums.length - 2; i++) {
        if (nums[i] > 0) {
            break;
        }
        if (i > 0 && nums[i] === nums[i - 1]) {
            continue;
        }
        for (let j = i + 1; j < nums.length - 1; j++) {
            if (j > i + 1 && nums[j] === nums[j - 1]) {
                continue;
            }
            let sum2 = nums[i] + nums[j];
            if (sum2 > 0 || -sum2 < nums[j]) {
                break;
            }
            if (-sum2 == nums[j]) {
                if (nums[i] !== nums[j]) {
                    if (counter.get(-sum2)! > 1) {
                        result.push([nums[i], nums[j], -sum2]);
                    }
                } else {
                    // both nums[i] and nums[j] are 0
                    if (counter.get(0)! > 2) {
                        result.push([0, 0, 0]);
                    }
                }
            } else if (counter.has(-sum2)) {
                result.push([nums[i], nums[j], -sum2]);
            }
        }
    }

    return result;
};

export { };