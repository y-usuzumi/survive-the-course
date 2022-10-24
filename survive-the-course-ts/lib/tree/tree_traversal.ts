import TreeNode from './treenode';

// Recursive methods

export function traversePreOrderRecursive(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }
    visitor(root);
    traversePreOrderRecursive(root.left, visitor);
    traversePreOrderRecursive(root.right, visitor);
}

export function traverseInOrderRecursive(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }
    traverseInOrderRecursive(root.left, visitor);
    visitor(root);
    traverseInOrderRecursive(root.right, visitor);
}

export function traversePostOrderRecursive(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }
    traversePostOrderRecursive(root.left, visitor);
    traversePostOrderRecursive(root.right, visitor);
    visitor(root);
}

// Iterative methods
export function traversePreOrderIterative(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }

    let stack: (TreeNode | null)[] = [];
    let curr: TreeNode | null = root;

    while (curr !== null || stack.length !== 0) {
        if (curr !== null) {
            visitor(curr);
            if (curr.left !== null) {
                stack.push(curr);
                curr = curr.left;
            } else {
                curr = curr.right;
            }
        } else {
            curr = stack.pop()!.right;
        }
    }
}

export function traverseInOrderIterative(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }

    let stack: (TreeNode | null)[] = [];
    let curr: TreeNode | null = root;

    while (curr !== null || stack.length !== 0) {
        if (curr !== null) {
            stack.push(curr);
            curr = curr.left;
            continue;
        } else {
            curr = stack.pop()!;
            visitor(curr);
            curr = curr.right;
        }
    }
}

export function traversePostOrderIterative(root: TreeNode | null, visitor: (node: TreeNode) => void) {
    if (root === null) {
        return;
    }

    let stack: (TreeNode | null)[] = [root];
    let reverseResult: (TreeNode | null)[] = [];
    while (stack.length > 0) {
        let node = stack.pop()!;
        reverseResult.push(node);
        if (node.left !== null) {
            stack.push(node.left);
        }
        if (node.right !== null) {
            stack.push(node.right);
        }
    }
    while (reverseResult.length > 0) {
        visitor(reverseResult.pop()!);
    }
}



function tests() {
    const tree = TreeNode.fromArray([1, 2, 3, 4, null, 5, 6, null, null, null, null, null, 7, null, 8]);
    {
        const result: number[] = [];
        traversePreOrderIterative(tree, node => result.push(node.val));
        console.log(`Preorder: ${result}`);
    }
    {
        const result: number[] = [];
        traversePreOrderRecursive(tree, node => result.push(node.val));
        console.log(`Preorder: ${result}`);
    }
    {
        const result: number[] = [];
        traverseInOrderIterative(tree, node => result.push(node.val));
        console.log(`Inorder: ${result}`);
    }
    {
        const result: number[] = [];
        traverseInOrderRecursive(tree, node => result.push(node.val));
        console.log(`Inorder: ${result}`);
    }
    {
        const result: number[] = [];
        traversePostOrderIterative(tree, node => result.push(node.val));
        console.log(`Postorder: ${result}`);
    }
    {
        const result: number[] = [];
        traversePostOrderRecursive(tree, node => result.push(node.val));
        console.log(`Postorder: ${result}`);
    }
}

tests();