function largestRectangleArea(heights: number[]): number {
    const stack: [number, number][] = [];
    let result = Number.MIN_SAFE_INTEGER;
    for (let idx = 0; idx < heights.length; idx++) {
        const height = heights[idx];
        let start = idx;
        while (stack.length > 0 && height < stack[stack.length - 1][0]) {
            let [h, hIdx] = stack.pop()!;
            const tempArea = h * (idx - hIdx);
            if (tempArea > result) {
                result = tempArea;
            }
            start = hIdx;
        }
        stack.push([height, start]);
    }
    for (const [h, hIdx] of stack) {
        const tempArea = h * (heights.length - hIdx);
        if (tempArea > result) {
            result = tempArea;
        }
    }
    return result;
};

export {};