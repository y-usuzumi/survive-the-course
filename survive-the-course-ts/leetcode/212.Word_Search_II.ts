// https://leetcode.com/problems/word-search-ii/
class TrieNode {
    val: string | null;
    children: Map<string, TrieNode>;
    parent: TrieNode | null;

    constructor(val?: string) {
        if (val === undefined) {
            val = null;
        }
        this.val = val;
        this.children = new Map();
    }

    addWord(s: string) {
        let curr: TrieNode = this;
        for (const ch of s) {
            let next = curr.children.get(ch);
            if (next === undefined) {
                next = new TrieNode();
                curr.children.set(ch, next);
            }
            curr = next;
        }
        curr.val = s;
    }
}

function findWords(board: string[][], words: string[]): string[] {
    if (board.length === 0 || board[0].length === 0) {
        return [];
    }

    if (words.length === 0) {
        return [];
    }

    const node = new TrieNode();
    for (const word of words) {
        node.addWord(word);
    }

    let visited = new Array(board.length);
    for (let idx = 0; idx < board.length; idx++) {
        visited[idx] = new Array(board[0].length).fill(false);
    }

    let result: Set<string> = new Set();

    for (let rowIdx = 0; rowIdx < board.length; rowIdx++) {
        for (let colIdx = 0; colIdx < board[0].length; colIdx++) {
            search(board, rowIdx, colIdx, node, visited, result);
        }
    }

    return Array.from(result);
}

function search(board: string[][], rowIdx: number, colIdx: number, node: TrieNode, visited: boolean[][], result: Set<string>): boolean {
    if (rowIdx < 0 || rowIdx >= board.length) {
        return false;
    }

    if (colIdx < 0 || colIdx >= board[0].length) {
        return false;
    }

    if (visited[rowIdx][colIdx]) {
        return false;
    }

    const ch = board[rowIdx][colIdx];
    const nextNode = node.children.get(ch);
    if (nextNode === undefined) {
        return;
    }

    if (nextNode.val !== null) {
        result.add(nextNode.val);
    }

    visited[rowIdx][colIdx] = true;

    if (search(board, rowIdx+1, colIdx, nextNode, visited, result)) {
        // if ()
    }
    search(board, rowIdx, colIdx+1, nextNode, visited, result);
    search(board, rowIdx-1, colIdx, nextNode, visited, result);
    search(board, rowIdx, colIdx-1, nextNode, visited, result);

    visited[rowIdx][colIdx] = false;
}

console.log(findWords([["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], ["oath","pea","eat","rain"]));

export {};