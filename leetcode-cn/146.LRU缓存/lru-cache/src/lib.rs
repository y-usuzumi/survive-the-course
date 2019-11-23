use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;

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
            hm: HashMap::with_capacity(capacity as usize),
            entry: None,
            exit: None,
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        unsafe {
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
                        let entryp = self.entry.unwrap();
                        let entry = &mut *entryp;
                        node.next = Some(entryp);
                        node.prev = None;
                        entry.prev = Some(node);
                        self.entry = Some(*vp);
                    }
                    node.val
                }
                None => -1,
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        unsafe {
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
                        let entryp = self.entry.unwrap();
                        let entry = &mut *entryp;
                        node.next = Some(entryp);
                        node.prev = None;
                        entry.prev = Some(node);
                        self.entry = Some(*vp);
                    }
                    node.val = value;
                }
                None => {
                    let nodep = alloc(Layout::new::<Node>()) as *mut Node;
                    let node = &mut *nodep;
                    node.key = key;
                    node.val = value;
                    if self.count > 0 {
                        let entryp = self.entry.unwrap();
                        let entry = &mut *entryp;
                        node.next = Some(entry);
                        entry.prev = Some(nodep);
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
    }

    fn clean(&mut self) {
        unsafe {
            while self.count > self.cap {
                if let Some(exitp) = self.exit {
                    let exit_node = &mut *exitp;
                    if let Some(prevp) = exit_node.prev {
                        let mut prev = &mut *prevp;
                        prev.next = None;
                        self.exit = exit_node.prev;
                    } else {
                        self.exit = None;
                    }
                    self.hm.remove(&exit_node.key);
                    // dealloc(exitp as *mut u8, Layout::new::<Node>());
                }
                self.count -= 1;
            }
        }
    }

    fn display(&self) {
        unsafe {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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

    #[test]
    fn it_works2() {
        let mut cache = LRUCache::new(2);
        cache.put(2, 1);
        cache.put(1, 1);
        cache.put(2, 3);
        cache.put(4, 1);
        cache.get(1);
        cache.get(2);
        cache.display();
    }

    #[test]
    fn it_works3() {
        // ["LRUCache","put","put","put","get","put","put","get","put","put","get","put","get","get","get","put","put","get","put","get"]
        // [[10],[7,28],[7,1],[8,15],[6],[10,27],[8,10],[8],[6,29],[1,9],[6],[10,7],[1],[2],[13],[8,30],[1,5],[1],[13,2],[12]]
        let mut cache = LRUCache::new(10);
        cache.put(7, 28);
        cache.put(7, 1);
        cache.put(8, 15);
        cache.get(6);
        cache.put(10, 27);
        cache.put(8, 10);
        cache.get(8);
    }

    fn leetcode_test(size: i32, cmds: Vec<&str>, args: Vec<Vec<i32>>) {
        let mut cache = LRUCache::new(size);
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
    fn leetcode_test1() {
        let cmds = vec![
            "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put",
            "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get",
            "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put",
            "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put",
            "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put",
            "get", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put",
            "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put",
            "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get",
            "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put",
            "put",
        ];
        let args = vec![
            vec![10, 13],
            vec![3, 17],
            vec![6, 11],
            vec![10, 5],
            vec![9, 10],
            vec![13],
            vec![2, 19],
            vec![2],
            vec![3],
            vec![5, 25],
            vec![8],
            vec![9, 22],
            vec![5, 5],
            vec![1, 30],
            vec![11],
            vec![9, 12],
            vec![7],
            vec![5],
            vec![8],
            vec![9],
            vec![4, 30],
            vec![9, 3],
            vec![9],
            vec![10],
            vec![10],
            vec![6, 14],
            vec![3, 1],
            vec![3],
            vec![10, 11],
            vec![8],
            vec![2, 14],
            vec![1],
            vec![5],
            vec![4],
            vec![11, 4],
            vec![12, 24],
            vec![5, 18],
            vec![13],
            vec![7, 23],
            vec![8],
            vec![12],
            vec![3, 27],
            vec![2, 12],
            vec![5],
            vec![2, 9],
            vec![13, 4],
            vec![8, 18],
            vec![1, 7],
            vec![6],
            vec![9, 29],
            vec![8, 21],
            vec![5],
            vec![6, 30],
            vec![1, 12],
            vec![10],
            vec![4, 15],
            vec![7, 22],
            vec![11, 26],
            vec![8, 17],
            vec![9, 29],
            vec![5],
            vec![3, 4],
            vec![11, 30],
            vec![12],
            vec![4, 29],
            vec![3],
            vec![9],
            vec![6],
            vec![3, 4],
            vec![1],
            vec![10],
            vec![3, 29],
            vec![10, 28],
            vec![1, 20],
            vec![11, 13],
            vec![3],
            vec![3, 12],
            vec![3, 8],
            vec![10, 9],
            vec![3, 26],
            vec![8],
            vec![7],
            vec![5],
            vec![13, 17],
            vec![2, 27],
            vec![11, 15],
            vec![12],
            vec![9, 19],
            vec![2, 15],
            vec![3, 16],
            vec![1],
            vec![12, 17],
            vec![9, 1],
            vec![6, 19],
            vec![4],
            vec![5],
            vec![5],
            vec![8, 1],
            vec![11, 7],
            vec![5, 2],
            vec![9, 28],
            vec![1],
            vec![2, 2],
            vec![7, 4],
            vec![4, 22],
            vec![7, 24],
            vec![9, 26],
            vec![13, 28],
            vec![11, 26],
        ];
        leetcode_test(10, cmds, args);
    }

    #[test]
    fn leetcode_test2() {
        let cmds = vec!["put", "put", "put", "put", "get", "get"];
        let args = vec![vec![2, 1], vec![1, 1], vec![2, 3], vec![4, 1], vec![1], vec![2]];
        leetcode_test(2, cmds, args);
    }
}
