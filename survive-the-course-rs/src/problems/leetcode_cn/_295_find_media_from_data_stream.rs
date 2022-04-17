// https://leetcode-cn.com/problems/find-median-from-data-stream/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MedianFinder {
    heapl: BinaryHeap<i32>,
    heapr: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    pub fn new() -> Self {
        MedianFinder { heapl: BinaryHeap::new(), heapr: BinaryHeap::new() }
    }
    
    pub fn add_num(&mut self, num: i32) {
        match self.heapr.peek() {
            None => {
                self.heapr.push(Reverse(num));
            },
            Some(&Reverse(val)) => {
                if num > val {
                    self.heapr.push(Reverse(num));
                } else {
                    self.heapl.push(num);
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
        if self.heapl.len() > self.heapr.len() {
            return *self.heapl.peek().unwrap() as f64;
        }

        if self.heapr.len() > self.heapl.len() {
            return self.heapr.peek().unwrap().0 as f64;
        }

        return ((*self.heapl.peek().unwrap() + self.heapr.peek().unwrap().0) as f64) / 2.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut medianFinder = MedianFinder::new();
        medianFinder.add_num(1);    // arr = [1]
        medianFinder.add_num(2);    // arr = [1, 2]
        assert_eq!(medianFinder.find_median(), 1.5); // return 1.) (i.e., (1 + 2) / 2)
        medianFinder.add_num(3);    // arr[1, 2, 3]
        assert_eq!(medianFinder.find_median(), 2.0); // return 2.0
        // assert_eq!(Solution::remove_nth_from_end(), 0);
    }
}
