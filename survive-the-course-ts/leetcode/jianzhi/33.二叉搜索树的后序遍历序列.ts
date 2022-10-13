// Problem link: https://leetcode.cn/problems/er-cha-sou-suo-shu-de-hou-xu-bian-li-xu-lie-lcof/?envType=study-plan&id=lcof

function verifyPostorder(postorder: number[]): boolean {
    return verifyPostorderWithIndex(postorder, 0, postorder.length);
};

function verifyPostorderWithIndex(arr: number[], start: number, end: number): boolean {
    if (end - start < 2) {
        return true;
    }

    let root = arr[end - 1];
    let rightStart = 0;

    while (arr[rightStart] < root) {
        rightStart++;
    }

    for (let rightIdx = rightStart; rightIdx < end - 1; rightIdx++) {
        if (arr[rightIdx] <= root) {
            return false;
        }
    }

    return verifyPostorderWithIndex(arr, start, rightStart) && verifyPostorderWithIndex(arr, rightStart, end - 1);
}

export {};