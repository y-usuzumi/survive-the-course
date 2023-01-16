// https://leetcode.com/problems/word-search-ii/

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    word: Option<String>,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
struct Trie {
    head: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut root = &mut self.head;
        for ch in word.chars() {
            root = root.children.entry(ch).or_insert(Default::default());
        }

        root.word = Some(word);
    }
}

pub struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn helper(
            board: &Vec<Vec<char>>,
            row_idx: usize,
            col_idx: usize,
            visited: &mut Vec<Vec<bool>>,
            curr_node: &mut TrieNode,
            result: &mut Vec<String>,
        ) {
            if visited[row_idx][col_idx] {
                return;
            }
            if let Some(node) = curr_node.children.get_mut(&board[row_idx][col_idx]) {
                if let Some(word) = &node.word {
                    result.push(word.clone());
                    node.word = None;
                }
                visited[row_idx][col_idx] = true;
                if row_idx > 0 {
                    helper(&board, row_idx - 1, col_idx, visited, node, result);
                }
                if col_idx > 0 {
                    helper(&board, row_idx, col_idx - 1, visited, node, result);
                }
                if row_idx < board.len() - 1 {
                    helper(&board, row_idx + 1, col_idx, visited, node, result);
                }
                if col_idx < board[0].len() - 1 {
                    helper(&board, row_idx, col_idx + 1, visited, node, result);
                }
                visited[row_idx][col_idx] = false;
            } else {
                return;
            }
        }

        let rows = board.len();
        let cols = board[0].len();
        if rows == 0 || cols == 0 {
            return vec![];
        }
        let mut visited = vec![vec![false; cols]; rows];
        let mut trie = Trie::new();
        for word in words {
            trie.add_word(word);
        }
        let mut result = vec![];

        for row_idx in 0..rows {
            for col_idx in 0..cols {
                helper(
                    &board,
                    row_idx,
                    col_idx,
                    &mut visited,
                    &mut trie.head,
                    &mut result,
                );
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
