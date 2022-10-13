class TrieNode {
    val: string | undefined;
    children: Map<string, TrieNode>;
    parent: TrieNode | undefined;
    constructor(val?: string, children?: Map<string, TrieNode>, parent?: TrieNode) {
        this.val = val;

        if (children === undefined) {
            this.children = new Map();
        } else {
            this.children = children;
        }
        
        this.parent = parent;
    }
}

const tn = new TrieNode();
console.log(tn.val);

class Trie {
    root: TrieNode;

    constructor() {
        this.root = new TrieNode();
    }

    insert(word: string): void {
        let currNode = this.root;
        for (const ch of word) {
            let nextNode;
            if (currNode.children.has(ch)) {
                nextNode = currNode.children.get(ch);
            } else {
                nextNode = new TrieNode(undefined, undefined, currNode);
                currNode.children.set(ch, nextNode);
            }
            currNode = nextNode;
        }
        currNode.val = word;
    }

    search(word: string): boolean {
        let currNode = this.root;
        for (const ch of word) {
            if (!currNode.children.has(ch)) {
                return false;
            }
            currNode = currNode.children.get(ch)!;
        }
        return currNode.val !== undefined;
    }

    startsWith(prefix: string): boolean {
        let currNode = this.root;
        for (const ch of prefix) {
            if (!currNode.children.has(ch)) {
                return false;
            }
            currNode = currNode.children.get(ch)!;
        }
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * var obj = new Trie()
 * obj.insert(word)
 * var param_2 = obj.search(word)
 * var param_3 = obj.startsWith(prefix)
 */
export {};