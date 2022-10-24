type SortFunc<T> = (a: T, b: T) => number;

function isEQ(a: number): boolean {
    return a === 0;
}

function isLT(a: number): boolean {
    return a < 0;
}

function isLE(a: number): boolean {
    return isEQ(a) || isLT(a);
}

function isGT(a: number): boolean {
    return !isLE(a);
}

function isGE(a: number): boolean {
    return !isLT(a);
}

class TreeNode<T> {
    private val: T;
    private left: TreeNode<T> | null;
    private right: TreeNode<T> | null
    private height: number;

    constructor(val: T) {
        this.val = val;
        this.left = null;
        this.right = null;
        this.height = 0;
    }

    private get leftHeight(): number {
        return this.left === null ? -1 : this.left.height;
    }

    private get rightHeight(): number {
        return this.right === null ? -1 : this.right.height;
    }

    private get balanceFactor(): number {
        return this.leftHeight - this.rightHeight;
    }

    private updateHeight() {
        this.height = 1 + Math.max(this.leftHeight, this.rightHeight);
    }

    private rotateR(): TreeNode<T> {
        const left = this.left;
        const leftRight = left?.right || null;
        this.left = leftRight;
        left!.right = this;
        this.updateHeight();
        left!.updateHeight();
        return left!;
    }

    private rotateL(): TreeNode<T> {
        const right = this.right;
        const rightLeft = right?.left || null;
        this.right = rightLeft;
        right!.left = this;
        this.updateHeight();
        right!.updateHeight();
        return right!;
    }

    private rotateLR(): TreeNode<T> {
        this.left = this.left!.rotateL();
        return this.rotateR();
    }

    private rotateRL(): TreeNode<T> {
        this.right = this.right!.rotateR();
        return this.rotateL();
    }

    private rebalance(): TreeNode<T> {
        const balanceFactor = this.balanceFactor;
        if (balanceFactor > 1) {
            // If balanceFactor > 1, left child is guaranteed to exist
            if (this.left!.balanceFactor > 0) {
                // Right rotation
                return this.rotateR();
            } else if (this.left!.balanceFactor < 0) {
                // Left-right rotation
                return this.rotateLR();
            } else {
                throw new Error("IMPOSSIBLE: balanceFactor > 1 while balanceFactor of left child is 0");
            }
        } else if (balanceFactor < -1) {
            // If balanceFactor < -1, right child is guaranteed to exist
            if (this.right!.balanceFactor < 0) {
                // Left rotation
                return this.rotateL();
            } else if (this.right!.balanceFactor > 0) {
                // Right-left rotation
                return this.rotateRL();
            } else {
                throw new Error("IMPOSSIBLE: balanceFactor < -1 while balanceFactor of right child is 0");
            }
        }
        return this;
    }

    private get minNode(): TreeNode<T> {
        if (this.left === null) {
            return this;
        }
        return this.left.minNode;
    }

    search(target: T, sortFunc: SortFunc<T>): TreeNode<T> | null {
        const sortResult = sortFunc(target, this.val);
        if (isEQ(sortResult)) {
            return this;
        }

        if (isLT(sortResult)) {
            return this.left === null ? null : this.left.search(target, sortFunc);
        }

        return this.right === null ? null : this.right.search(target, sortFunc);
    }

    insert(target: T, sortFunc: SortFunc<T>): TreeNode<T> {
        const sortResult = sortFunc(target, this.val);
        if (isEQ(sortResult)) {
            // Do not allow duplicate values
            return this;
        }
        if (isLT(sortResult)) {
            if (this.left === null) {
                this.left = new TreeNode(target);
            } else {
                this.left = this.left.insert(target, sortFunc);
            }
        } else {
            if (this.right === null) {
                this.right = new TreeNode(target);
            } else {
                this.right = this.right.insert(target, sortFunc);
            }
        }
        this.updateHeight();
        return this.rebalance();
    }

    remove(target: T, sortFunc: SortFunc<T>): TreeNode<T> | null {
        const sortResult = sortFunc(target, this.val);
        if (isEQ(sortResult)) {
            if (this.left === null) {
                const result = this.right;
                this.right = null;
                return result;
            } else if (this.right === null) {
                const result = this.left;
                this.left = null;
                return result;
            } else {
                const rightMinNode = this.right.minNode;
                const newRight = this.right.remove(rightMinNode.val, sortFunc);
                rightMinNode.left = this.left;
                rightMinNode.right = newRight;
                this.left = null;
                this.right = null;
                rightMinNode.updateHeight();
                return rightMinNode.rebalance();
            }
        }
        if (isLT(sortResult)) {
            this.left = this.left!.remove(target, sortFunc);

        } else if (isGT(sortResult)) {
            this.right = this.right!.remove(target, sortFunc);
        }
        this.updateHeight();
        return this.rebalance();
    }
}