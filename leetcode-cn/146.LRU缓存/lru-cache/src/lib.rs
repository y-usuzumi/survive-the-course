use std::collections::HashMap;
use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
}

#[derive(Debug)]
struct LRUCache {
    cap: i32,
    count: i32,
    hm: HashMap<i32, *mut Node>,
    entry: Option<*mut Node>,
    exit: Option<*mut Node>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity,
            count: 0,
            hm: HashMap::new(),
            entry: None,
            exit: None,
        }
    }
    unsafe fn get(&mut self, key: i32) -> i32 {
        match self.hm.get(&key) {
            Some(vp) => {
                let node = &mut **vp;
                if let Some(prevp) = node.prev {
                    let prev = &mut *prevp;
                    match node.next {
                        Some(nextp) => {
                            let next = &mut *nextp;
                            prev.next = Some(nextp);
                            next.prev = Some(prevp);
                        }
                        None => {
                            prev.next = None;
                            self.exit = Some(prevp);
                        }
                    }
                    let entry = self.entry.unwrap();
                    node.next = Some(entry);
                    self.entry = Some(*vp);
                }
                node.val
            }
            None => -1,
        }
    }

    unsafe fn put(&mut self, key: i32, value: i32) {
        match self.hm.get(&key) {
            Some(vp) => {
                let node = &mut **vp;
                if let Some(prevp) = node.prev {
                    let prev = &mut *prevp;
                    match node.next {
                        Some(nextp) => {
                            let next = &mut *nextp;
                            prev.next = Some(nextp);
                            next.prev = Some(prevp);
                        }
                        None => {
                            prev.next = None;
                            self.exit = Some(prevp);
                        }
                    }
                    let entry = self.entry.unwrap();
                    node.next = Some(entry);
                    self.entry = Some(*vp);
                }
                node.val = value;
            }
            None => {
                let mut nodep = alloc(Layout::new::<Node>()) as *mut Node;
                let node = &mut *nodep;
                node.key = key;
                node.val = value;
                if self.count > 0 {
                    let entryp = self.entry.unwrap();
                    let entry = &mut *entryp;
                    node.next = Some(entry);
                    self.entry = Some(nodep);
                } else {
                    self.entry = Some(nodep);
                    self.exit = Some(nodep);
                }
                self.hm.insert(key, nodep);
                self.count += 1;
                self.clean()
            }
        }
    }

    unsafe fn clean(&mut self) {
        if self.count > self.cap {
            if let Some(exitp) = self.exit {
                let exit_node = &mut *exitp;
                if let Some(prevp) = exit_node.prev {
                    let mut prev = &mut *prevp;
                    prev.next = None;
                }
                self.exit = exit_node.prev;
                self.hm.remove(&exit_node.key);
            }
            self.count -= 1;
        }
    }

    unsafe fn display(&self) {
        println!("Cache count/cap: {:?}/{:?}", self.count, self.cap);
        println!("Cache map:");
        for (k, v) in &self.hm {
            println!("- {:?} -> {:?}", k, (**v).val);
        }
        println!("Cache node:");
        println!("Entry -> {:?}", self.entry);
        println!("Exit -> {:?}", self.exit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let mut cache = LRUCache::new(2);
            cache.put(1, 1);
            cache.put(2, 2);
            cache.display();
            assert_eq!(cache.get(1), 1); // 返回  1
            cache.put(3, 3); // 该操作会使得密钥 2 作废
            assert_eq!(cache.get(2), -1); // 返回 -1 (未找到)
            cache.put(4, 4); // 该操作会使得密钥 1 作废
            assert_eq!(cache.get(1), -1); // 返回 -1 (未找到)
            assert_eq!(cache.get(3), 3); // 返回  3
            assert_eq!(cache.get(4), 4); // 返回  4
        }
    }

    // #[test]
    // fn it_works2() {
    //     unsafe {
    //         let mut cache = LRUCache::new(2);
    //         cache.put(2, 1);
    //         cache.put(1, 1);
    //         cache.put(2, 3);
    //         cache.put(4, 1);
    //         cache.get(1);
    //         cache.get(2);
    //         cache.display();
    //     }
    // }

    // #[test]
    // fn it_works3() {
    //     unsafe {
    //         // ["LRUCache","put","put","put","get","put","put","get","put","put","get","put","get","get","get","put","put","get","put","get"]
    //         // [[10],[7,28],[7,1],[8,15],[6],[10,27],[8,10],[8],[6,29],[1,9],[6],[10,7],[1],[2],[13],[8,30],[1,5],[1],[13,2],[12]]
    //         let mut cache = LRUCache::new(10);
    //         cache.put(7, 28);
    //         cache.put(7, 1);
    //         cache.put(8, 15);
    //         cache.get(6);
    //         cache.put(10, 27);
    //         cache.put(8, 10);
    //         cache.get(8);
    //     }
    // }
}
