// https://leetcode.com/problems/range-module/

use std::collections::BTreeMap;

struct RangeModule {
    ranges: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut new_right = right;
        let highest = self.highest(left);
        let mut query_left = left;
        if let Some((hl, hr)) = highest {
            if hr >= query_left {
                query_left = hl;
            }
        }
        let ranges: Vec<(i32, i32)> = self
            .ranges
            .range(query_left..=right)
            .map(|(l, r)| (*l, *r))
            .collect::<Vec<(i32, i32)>>();
        for (rngl, rngr) in ranges {
            self.ranges.remove(&rngl);
            new_right = new_right.max(rngr);
        }

        self.ranges.insert(query_left, new_right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let mut iter = self.highest(left);
        if let Some((rngl, rngr)) = iter {
            return right <= rngr;
        }
        return false;
    }

    fn highest(&self, left: i32) -> Option<(i32, i32)> {
        let mut iter = self.ranges.range(0..=left).rev().next();
        if let Some((rngl, rngr)) = iter {
            return Some((*rngl, *rngr));
        }
        return None;
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let highest = self.highest(left);
        let mut fixup_left = None;
        let mut range_left = left;
        if let Some((rngl, rngr)) = highest {
            if rngr > left {
                fixup_left = Some(rngl);
                range_left = rngl;
            }
        }

        let ranges: Vec<(i32, i32)> = self
            .ranges
            .range(range_left..right)
            .map(|(l, r)| (*l, *r))
            .collect();

        let mut max_right = right;
        for (rngl, rngr) in ranges {
            self.ranges.remove(&rngl);
            max_right = max_right.max(rngr);
        }

        let mut fixup_right = None;
        if max_right > right {
            fixup_right = Some(max_right);
        }

        if let Some(fixup_left) = fixup_left {
            self.add_range(fixup_left, left);
        }

        if let Some(fixup_right) = fixup_right {
            self.add_range(right, fixup_right);
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut rm = RangeModule::new();
        rm.add_range(10, 20);
        rm.remove_range(14, 16);
        assert!(rm.query_range(10, 14));
        assert!(!rm.query_range(13, 15));
        assert!(rm.query_range(16, 17));
    }
}
