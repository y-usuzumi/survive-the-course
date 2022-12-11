// https://leetcode.com/problems/guess-the-word/

use std::collections::HashSet;

pub struct Solution;

/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */

struct Master;
impl Master {
    fn guess(&self, word: String) -> usize {
        unimplemented!("Actual implementation is provided by OJ");
    }
}

impl Solution {
    const WORD_LEN: usize = 6;
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let match_map = Self::generate_match_map(&words);
        let mut remaining_words = HashSet::from_iter(0..words.len());
        loop {
            if remaining_words.is_empty() {
                println!("No remaining words!");
                return;
            }
            let (best_guess, mut best_groups) = Self::find_best_guess(&match_map, &remaining_words);
            match master.guess(words[best_guess].clone()) {
                Self::WORD_LEN => {
                    return;
                }
                group_idx => {
                    let best_group = best_groups.remove(group_idx);
                    remaining_words = HashSet::from_iter(best_group.into_iter());
                }
            }
        }
    }

    fn find_best_guess(
        match_map: &Vec<Vec<usize>>,
        remaining_words: &HashSet<usize>,
    ) -> (usize, Vec<Vec<usize>>) {
        let mut best_max_group_size = usize::MAX;
        let mut best_guess = 0;
        let mut best_groups = Vec::new();
        for &guess in remaining_words {
            let mut groups = vec![vec![]; Self::WORD_LEN + 1];
            let mut max_group_size = 0;
            for &word_idx in remaining_words {
                let group_idx = match_map[guess][word_idx];
                groups[group_idx].push(word_idx);
                max_group_size = max_group_size.max(groups[group_idx].len());
            }
            if max_group_size < best_max_group_size {
                best_max_group_size = max_group_size;
                best_guess = guess;
                best_groups = groups;
            }
        }

        return (best_guess, best_groups);
    }

    fn generate_match_map(words: &[String]) -> Vec<Vec<usize>> {
        let mut match_map = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() {
            for j in i..words.len() {
                let matched_chars = Self::get_matched_chars(&words[i], &words[j]);
                match_map[i][j] = matched_chars;
                match_map[j][i] = matched_chars;
            }
        }

        return match_map;
    }

    fn get_matched_chars(a: &str, b: &str) -> usize {
        return a.chars().zip(b.chars()).filter(|(a, b)| a == b).count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
