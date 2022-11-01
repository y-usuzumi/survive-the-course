// Problem link: https://leetcode.com/problems/gas-station/

function canCompleteCircuit(gas: number[], cost: number[]): number {
    const gains = new Array(gas.length);
    for (let idx = 0; idx < gas.length; idx++) {
        gains[idx] = gas[idx] - cost[idx];
    }
    
    let sum = 0;
    for (let idx = 0; idx < gains.length; idx++) {
        sum += gains[idx];
    }
    if (sum < 0) {
        return -1;
    }
    
    let currIdx = 0;
    let currGas = 0;
    for (let idx = 0; idx < gains.length; idx++) {
        currGas += gains[idx];
        if (currGas < 0) {
            currGas = 0;
            currIdx = idx+1;
            continue;
        }
    }
    return currIdx;
};

export {};