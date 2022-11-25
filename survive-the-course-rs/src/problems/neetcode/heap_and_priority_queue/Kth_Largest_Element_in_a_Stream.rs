// https://leetcode.com/problems/kth-largest-element-in-a-stream/

use std::iter::FromIterator;
use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_by(|a, b| b.cmp(a));
        Self {
            k: k as usize,
            heap: BinaryHeap::from_iter(nums.into_iter().map(|num| Reverse(num)).take(k as usize)),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k || val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
        }

        while self.heap.len() > self.k {
            self.heap.pop();
        }
        return self.heap.peek().unwrap().0;
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
