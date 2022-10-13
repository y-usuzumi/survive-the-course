namespace UseStack {
    function trap(height: number[]): number {
        if (height.length < 3) {
            return 0;
        }
    
        let stack = [[0, height[0]]];
        let total = 0;
        for (let rightIdx = 1; rightIdx < height.length; rightIdx++) {
            let filledHeight = 0;
            let rightHeight = height[rightIdx];
    
            while (stack.length > 0) {
                let lowest = stack[stack.length - 1];
                if (rightHeight >= lowest[1]) {
                    stack.pop();
                    total += (rightIdx - lowest[0] - 1) * (lowest[1] - filledHeight);
                    filledHeight = lowest[1];
                } else {
                    total += (rightIdx - lowest[0] - 1) * (rightHeight - filledHeight);
                    filledHeight = 0;
                    break;
                }
            }
            stack.push([rightIdx, rightHeight]);
            filledHeight = 0;
        }
    
        return total;
    };
}

namespace UseTwoPointers {
    function trap(height: number[]): number {
        if (height.length < 3) {
            return 0;
        }

        let result = 0;

        let leftIdx = 0;
        let rightIdx = height.length - 1;
        let currMinHeight = 0;

        while (leftIdx < rightIdx) {
            let leftHeight = height[leftIdx];
            let rightHeight = height[rightIdx];
            
            if (leftHeight < rightHeight) {
                if (leftHeight < currMinHeight) {
                    const filled = currMinHeight - leftHeight;
                    result += filled;
                } else {
                    currMinHeight = leftHeight;
                }
                leftIdx++;
            } else {
                if (rightHeight < currMinHeight) {
                    const filled = currMinHeight - rightHeight;
                    result += filled;
                } else {
                    currMinHeight = Math.max(currMinHeight, rightHeight);
                }
                rightIdx--;
            }
        }

        return result;
    }
}

export {};