// https://leetcode.com/problems/my-calendar-i/

use std::collections::BTreeMap;

struct MyCalendar {
    books: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            books: BTreeMap::new(),
        }
    }

    // Use a tree map to hold start:end information
    // For each incoming booking request, find in the tree map the largest
    // start less than the requested end (a).
    // Ensure the end of (a) is less than the start.
    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut iterator_prev = self.books.range(0..end).rev();
        match iterator_prev.next() {
            Some(prev) => {
                if *prev.1 > start {
                    return false;
                }
            }
            None => {}
        }

        self.books.insert(start, end);
        return true;
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(cal.book(20, 30));
    }
}
