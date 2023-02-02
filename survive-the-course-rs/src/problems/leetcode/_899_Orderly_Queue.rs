// https://leetcode.com/problems/orderly-queue/

pub struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if k > 1 {
            chars.sort();
            return chars.into_iter().collect();
        }

        let mut min_chars = chars.clone();

        for _ in 0..chars.len() {
            chars.rotate_left(1);
            if chars < min_chars {
                min_chars = chars.clone();
            }
        }

        return min_chars.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
