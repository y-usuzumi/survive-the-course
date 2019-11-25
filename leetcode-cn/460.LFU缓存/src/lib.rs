use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::collections::HashMap;
use std::ptr::null_mut;

#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
    slot: *mut SlotNode,
}

#[derive(Debug)]
struct SlotNode {
    hits: i32,
    prev: *mut SlotNode,
    next: *mut SlotNode,
    node_entry: *mut Node,
    node_exit: *mut Node,
}

struct LFUCache {
    cap: usize,
    count: usize,
    hm: HashMap<i32, *mut Node>,
    slot_entry: *mut SlotNode,
}

impl Node {
    fn incr(&mut self) {
        unsafe {
            let slotp = self.slot;
            let slot = &mut *slotp;
            let incr_slotp = slot.get_or_create_incr_slot_nodep();
            let incr_slot = &mut *incr_slotp;
            slot.remove_node(self);
            incr_slot.push_node(self);
        }
    }
}

impl SlotNode {
    fn get_or_create_incr_slot_nodep(&mut self) -> *mut Self {
        unsafe {
            if self.prev.is_null() {
                let incr_slotp = alloc_zeroed(Layout::new::<Self>()) as *mut Self;
                let incr_slot = &mut *incr_slotp;
                incr_slot.hits = self.hits + 1;
                incr_slot.prev = null_mut();
                incr_slot.next = self;
                // println!("Created slot with hits: {:?}", incr_slot.hits);
                self.prev = incr_slot;
                incr_slot
            } else {
                let prev = &mut *self.prev;
                if prev.hits > self.hits + 1 {
                    let incr_slotp = alloc_zeroed(Layout::new::<Self>()) as *mut Self;
                    let incr_slot = &mut *incr_slotp;
                    incr_slot.hits = self.hits + 1;
                    incr_slot.prev = self.prev;
                    incr_slot.next = self;
                    prev.next = incr_slot;
                    self.prev = incr_slot;
                    incr_slot
                } else {
                    self.prev
                }
            }
        }
    }

    fn push_node(&mut self, nodep: *mut Node) {
        unsafe {
            let node = &mut *nodep;
            node.next = self.node_entry;
            node.prev = null_mut();
            node.slot = self;
            if !self.node_entry.is_null() {
                let node_entry = &mut *self.node_entry;
                node_entry.prev = nodep;
            }
            self.node_entry = nodep;
            if self.node_exit.is_null() {
                self.node_exit = nodep;
            }
        }
    }

    fn evict_node(&mut self) -> *mut Node {
        unsafe {
            let nodep = self.node_exit;
            let node = &mut *nodep;
            node.slot = null_mut();
            if node.prev.is_null() {
                node.prev = null_mut();
                self.node_entry = null_mut();
                self.node_exit = null_mut();
            } else {
                let prev = &mut *node.prev;
                prev.next = null_mut();
                self.node_exit = prev;
                node.prev = null_mut();
            }
            nodep
        }
    }

    fn remove_node(&mut self, nodep: *mut Node) -> *mut Node {
        unsafe {
            // println!("Remove node 1");
            let node = &mut *nodep;
            // assert_eq!(node.slot, self);
            if node.prev.is_null() {
                // println!("Remove node 1.1");
                self.node_entry = node.next
            } else {
                // !("Remove node 1.2");
                let prev = &mut *node.prev;
                prev.next = node.next;
            }
            if node.next.is_null() {
                // println!("Remove node 2.1");
                self.node_exit = node.prev;
            } else {
                // println!("Remove node 2.2");
                let next = &mut *node.next;
                next.prev = node.prev;
            }
            node.prev = null_mut();
            node.next = null_mut();
            node.slot = null_mut();
            nodep
        }
    }

    fn is_empty(&self) -> bool {
        self.node_entry.is_null()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        unsafe {
            let slotp = alloc_zeroed(Layout::new::<SlotNode>()) as *mut SlotNode;
            let slot = &mut *slotp;
            slot.hits = 1;
            // println!("Created slot with hits: {:?}", slot.hits);
            LFUCache {
                cap: capacity as usize,
                count: 0,
                hm: HashMap::default(),
                slot_entry: slotp,
            }
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        unsafe {
            match self.hm.get_mut(&key) {
                Some(&mut nodep) => {
                    let node = &mut *nodep;
                    node.incr();
                    node.val
                }
                None => -1,
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            match self.hm.get_mut(&key) {
                Some(&mut nodep) => {
                    let node = &mut *nodep;
                    node.incr();
                    node.val = value;
                }
                None => {
                    if self.cap == 0 {
                        return
                    }
                    if self.count >= self.cap {
                        self.clean();
                        self.count -= 1;
                    }
                    let nodep = alloc_zeroed(Layout::new::<Node>()) as *mut Node;
                    let node = &mut *nodep;
                    node.key = key;
                    node.val = value;
                    let slot = &mut *self.slot_entry;
                    slot.push_node(node);
                    self.hm.insert(key, nodep);
                    self.count += 1;
                }
            }
        }
    }

    fn clean(&mut self) {
        unsafe {
            let mut slotp = self.slot_entry;
            loop {
                if slotp.is_null() {
                    unreachable!();
                }
                let slot = &mut *slotp;
                if slot.is_empty() {
                    slotp = slot.prev;
                } else {
                    break;
                }
            }
            let slot = &mut *slotp;
            let nodep = slot.evict_node();
            let node = &mut *nodep;
            // println!("Node {:?} => {:?} evicted", node.key, node.val);
            self.hm.remove(&node.key);
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    fn leetcode_test(size: i32, cmds: Vec<&str>, args: Vec<Vec<i32>>) {
        let mut cache = LFUCache::new(size);
        for (cmd, arg) in cmds.iter().zip(args) {
            match cmd.as_ref() {
                "get" => {
                    let r = cache.get(arg[0]);
                    println!("Getting {:?} => {:?}", arg, r);
                }
                "put" => {
                    println!("Putting {:?}", arg);
                    cache.put(arg[0], arg[1]);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_1() {
        let cmds = vec![
            "put", "put", "get", "put", "get", "get", "put", "get", "get", "get",
        ];
        let args = vec![
            vec![1, 1],
            vec![2, 2],
            vec![1],    // returns 1
            vec![3, 3], // evicts key 2
            vec![2],    // returns -1 (not found)
            vec![3],    // returns 3.
            vec![4, 4], // evicts key 1.
            vec![1],    // returns -1 (not found)
            vec![3],    // returns 3
            vec![4],    // returns 4
        ];
        leetcode_test(2, cmds, args);
    }

    #[test]
    fn test_2() {
        let cmds = vec!["put", "get"];
        let args = vec![vec![0, 0], vec![0]];
        leetcode_test(0, cmds, args);
    }
}
