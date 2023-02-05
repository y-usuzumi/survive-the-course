// https://leetcode.com/problems/maximum-frequency-stack/

use std::collections::HashMap;

struct FreqStack {
    max_freq: usize,
    num_freq_map: HashMap<i32, usize>,
    freq_num_map: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            max_freq: 0,
            num_freq_map: HashMap::new(),
            freq_num_map: vec![vec![]],
        }
    }

    fn push(&mut self, val: i32) {
        let freq = self.num_freq_map.entry(val).or_default();
        *freq += 1;
        if *freq >= self.freq_num_map.len() {
            self.freq_num_map.push(vec![]);
        }
        self.freq_num_map[*freq].push(val);
        self.max_freq = self.max_freq.max(*freq);
    }

    fn pop(&mut self) -> i32 {
        let result = self.freq_num_map[self.max_freq].pop().unwrap();
        if self.freq_num_map[self.max_freq].len() == 0 {
            self.max_freq -= 1;
            self.freq_num_map.pop();
        }
        *self.num_freq_map.get_mut(&result).unwrap() -= 1;
        return result;
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

#[cfg(test)]
mod tests {
    use super::*;
}
