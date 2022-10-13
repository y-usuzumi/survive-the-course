function containsDuplicate(nums: number[]): boolean {
    let s = new Set();
    for (const num of nums) {
        if (s.has(num)) {
            return false;
        }
    }
    return true;
};

