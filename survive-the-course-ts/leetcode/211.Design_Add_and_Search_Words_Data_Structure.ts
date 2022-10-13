class DictionaryNode {
    val: string | null;
    children: Map<string, DictionaryNode>;
    parent: DictionaryNode | null;

    constructor(val?: string, children?: Map<string, DictionaryNode>, parent?: DictionaryNode) {
        this.val = val ?? null;
        this.children = children ?? new Map();
        this.parent = parent ?? null;
    }
}

class WordDictionary {
    private root: DictionaryNode;

    constructor() {
        this.root = new DictionaryNode();
    }

    addWord(word: string): void {
        let curr = this.root;
        for (const ch of word) {
            if (curr.children.has(ch)) {
                curr = curr.children.get(ch)!;
            } else {
                let next = new DictionaryNode(undefined, undefined, curr);
                curr.children.set(ch, next);
                curr = next;
            }
            
        }
        curr.val = word;
    }

    search(word: string): boolean {
        function searchRec(node: DictionaryNode | null, chars: string, idx: number): boolean {
            if (node === null) {
                return false;
            }

            if (idx >= chars.length) {
                return node.val !== null;
            }

            const ch = chars[idx];
            if (ch === '.') {
                for (const [key, next] of node.children) {
                    if (searchRec(next, chars, idx+1)) {
                        return true;
                    }
                }
                return false;
            } else {
                return searchRec(node.children.get(ch) ?? null, chars, idx+1);
            }
        }
        return searchRec(this.root, word, 0);
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * var obj = new WordDictionary()
 * obj.addWord(word)
 * var param_2 = obj.search(word)
 */

export {};