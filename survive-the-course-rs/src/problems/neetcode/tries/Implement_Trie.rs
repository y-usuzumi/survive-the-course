// https://leetcode.com/problems/implement-trie-prefix-tree/

use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    val: Option<String>,
    children: HashMap<char, TrieNode>,
}

#[derive(Debug)]
struct Trie {
    head: TrieNode,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            val: None,
            children: HashMap::new(),
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            head: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut root = &mut self.head;
        let mut cursor = 0;
        let chars: Vec<char> = word.chars().collect();
        while cursor < chars.len() {
            let char = chars[cursor];
            root = root.children.entry(char).or_insert(TrieNode::new());
            cursor += 1;
        }
        root.val = Some(word);
    }

    fn search(&self, word: String) -> bool {
        let mut root = &self.head;
        let mut cursor = 0;
        let chars: Vec<char> = word.chars().collect();
        while cursor < chars.len() {
            let char = chars[cursor];
            if let Some(node) = root.children.get(&char) {
                root = node;
            } else {
                return false;
            }
            cursor += 1;
        }

        return root.val.is_some();
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut root = &self.head;
        let mut cursor = 0;
        let chars: Vec<char> = prefix.chars().collect();
        while cursor < chars.len() {
            let char = chars[cursor];
            if let Some(node) = root.children.get(&char) {
                root = node;
            } else {
                return false;
            }
            cursor += 1;
        }

        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("Hell".to_string());
        trie.insert("Hello".to_string());
        trie.insert("Human".to_string());
        println!("{:?}", trie);
    }

    #[test]
    fn test_search() {
        let mut trie = Trie::new();
        trie.insert("Hell".to_string());
        trie.insert("Hello".to_string());
        trie.insert("Human".to_string());
        assert!(trie.search("Hello".to_string()));
        assert!(!trie.search("Hel".to_string()));
    }
}
