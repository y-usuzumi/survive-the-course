class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }

    static fromArray(vals: (number | null)[]): TreeNode | null {
        if (vals.length === null) {
            return null;
        }
        let root = new TreeNode(vals[0]!);
        let currLayer: (TreeNode | null)[] = [root];
        let idx = 1;
        while (idx < vals.length) {
            let nextLayer: (TreeNode | null)[] = [];
            for (const node of currLayer) {
                let val = vals[idx++];
                if (node === null || val === null) {
                    nextLayer.push(null);
                } else {
                    node.left = new TreeNode(val);
                    nextLayer.push(node.left);
                }
                val = vals[idx++];
                if (node === null || val === null) {
                    nextLayer.push(null);
                } else {
                    node.right = new TreeNode(val);
                    nextLayer.push(node.right);
                }
            }
            currLayer = nextLayer;
        }
        return root;
    }
}

export default TreeNode;