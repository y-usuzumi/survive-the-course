// A DT-based solution
use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut idx_lookup: HashMap<usize, HashMap<usize, bool>> = HashMap::new();

        for (idx, _) in s.chars().enumerate() {
            let mut span_lookup = HashMap::new();
            span_lookup.insert(0, true);
            idx_lookup.insert(idx, span_lookup);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
