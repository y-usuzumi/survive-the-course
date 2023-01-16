// https://leetcode.com/problems/last-stone-weight/

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut h = BinaryHeap::new();
        for stone in stones {
            h.push(stone);
        }

        while h.len() > 1 {
            let s1 = h.pop().unwrap();
            let s2 = h.pop().unwrap();
            if s1 != s2 {
                h.push((s1 - s2).abs());
            }
        }

        return if h.is_empty() { 0 } else { h.pop().unwrap() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
