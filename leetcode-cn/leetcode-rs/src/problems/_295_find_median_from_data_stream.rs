// https://leetcode-cn.com/problems/find-median-from-data-stream/
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MedianFinder {
    heapl: BinaryHeap<i32>,
    heapr: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            heapl: BinaryHeap::new(),
            heapr: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.heapl.is_empty() {
            self.heapr.push(Reverse(num));
        } else {
            match self.heapr.peek() {
                Some(&Reverse(top)) => {
                    if num <= top {
                        self.heapl.push(num);
                    } else {
                        self.heapr.push(Reverse(num));
                    }
                }
                None => {
                    panic!("Impossible")
                }
            }
        }

        if self.heapl.len() == self.heapr.len() + 2 {
            self.heapr.push(Reverse(self.heapl.pop().unwrap()));
        } else if self.heapr.len() == self.heapl.len() + 2 {
            self.heapl.push(self.heapr.pop().unwrap().0);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.heapl.len() < self.heapr.len() {
            return self.heapr.peek().unwrap().0 as f64;
        }

        if self.heapl.len() > self.heapr.len() {
            return *self.heapl.peek().unwrap() as f64;
        }

        return (*self.heapl.peek().unwrap() + self.heapr.peek().unwrap().0) as f64 / 2.;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        median_finder.add_num(2); // arr = [1, 2]
        assert_eq!(median_finder.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3); // arr[1, 2, 3]
        assert_eq!(median_finder.find_median(), 2.0); // return 2.0
    }
}
