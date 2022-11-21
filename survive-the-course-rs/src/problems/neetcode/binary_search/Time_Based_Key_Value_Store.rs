// https://leetcode.com/problems/time-based-key-value-store/

use std::collections::{hash_map::Entry, HashMap};

struct TimeMap {
    hm: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self { hm: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.hm.entry(key) {
            Entry::Vacant(e) => {
                e.insert(vec![(timestamp, value)]);
            }
            Entry::Occupied(e) => {
                e.into_mut().push((timestamp, value));
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.hm.contains_key(&key) {
            return "".to_string();
        }

        let arr = self.hm.get(&key).unwrap();
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = (left + right - 1) / 2;
            if arr[mid].0 > timestamp {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if right == 0 {
            return "".to_string();
        }
        return arr[right - 1].1.clone();
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut tm = TimeMap::new();
        tm.set("foo".into(), "bar".into(), 1);
        assert_eq!(tm.get("foo".into(), 1), "bar".to_string());
        assert_eq!(tm.get("foo".into(), 3), "bar".to_string());
        tm.set("foo".into(), "bar2".into(), 4);
        assert_eq!(tm.get("foo".into(), 4), "bar2".to_string());
        assert_eq!(tm.get("foo".into(), 5), "bar2".to_string());
    }
}
