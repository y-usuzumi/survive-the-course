import { TreeNode } from '../lib'

function morrisTraversal(node: TreeNode | null): number[] {
    let curr = node;
    let result: number[] = [];
    while (curr !== null) {
        if (curr.left !== null) {
            let attachTo = curr.left;
            while (attachTo.right !== null && attachTo.right !== curr) {
                attachTo = attachTo.right;
            }
            if (attachTo.right === null) {
                attachTo.right = curr;
                curr = curr.left;
            } else {
                attachTo.right = null;
                result.push(curr.val);
                curr = curr.right;
            }
        } else {
            result.push(curr.val);
            curr = curr.right;
        }
    }
    return result;
}

const tree = TreeNode.fromArray([1, 2, 3, 4, 5, null, null]);
console.log(morrisTraversal(tree));