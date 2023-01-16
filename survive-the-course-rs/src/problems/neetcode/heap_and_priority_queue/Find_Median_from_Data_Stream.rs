use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct MedianFinder {
    lower_heap: BinaryHeap<i32>,
    upper_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.lower_heap.is_empty() {
            self.lower_heap.push(num);
            return;
        }
        if num <= *self.lower_heap.peek().unwrap() {
            self.lower_heap.push(num);
            if self.lower_heap.len() - self.upper_heap.len() >= 2 {
                self.upper_heap
                    .push(Reverse(self.lower_heap.pop().unwrap()));
            }
        } else {
            self.upper_heap.push(Reverse(num));
            if self.upper_heap.len() - self.lower_heap.len() >= 2 {
                self.lower_heap.push(self.upper_heap.pop().unwrap().0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        match self.lower_heap.len().cmp(&self.upper_heap.len()) {
            std::cmp::Ordering::Equal => {
                (self.lower_heap.peek().unwrap() + self.upper_heap.peek().unwrap().0) as f64 / 2.0
            }
            std::cmp::Ordering::Greater => *self.lower_heap.peek().unwrap() as f64,
            std::cmp::Ordering::Less => self.upper_heap.peek().unwrap().0 as f64,
        }
    }
}
