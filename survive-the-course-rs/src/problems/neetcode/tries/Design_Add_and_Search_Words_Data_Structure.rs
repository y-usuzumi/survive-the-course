// https://leetcode.com/problems/design-add-and-search-words-data-structure/

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    has_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
struct WordDictionary {
    head: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut root = &mut self.head;
        for ch in word.chars() {
            root = root.children.entry(ch).or_insert(Default::default());
        }

        root.has_word = true;
    }

    fn search(&self, word: String) -> bool {
        fn helper(mut root: &TrieNode, word: &[char]) -> bool {
            for (idx, ch) in word.iter().enumerate() {
                match ch {
                    '.' => {
                        for (_, next) in &root.children {
                            if helper(next, &word[idx + 1..]) {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => {
                        if let Some(node) = root.children.get(&ch) {
                            root = node;
                        } else {
                            return false;
                        }
                    }
                }
            }
            return root.has_word;
        }
        return helper(&self.head, &word.chars().collect::<Vec<char>>());
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut wd = WordDictionary::new();
        for word in ["bad", "dad", "mad"] {
            wd.add_word(word.to_string());
        }
        assert!(wd.search("b..".to_string()));
    }
}
