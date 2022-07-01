// https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/

struct CQueue {
    forward_stack: Vec<i32>,
    backward_stack: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        CQueue {
            forward_stack: Vec::new(),
            backward_stack: Vec::new(),
        }
    }

    fn dump(&mut self) {
        while !self.forward_stack.is_empty() {
            self.backward_stack.push(self.forward_stack.pop().unwrap());
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.forward_stack.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.backward_stack.is_empty() {
            self.dump();
        }
        self.backward_stack.pop().unwrap_or(-1)
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut queue = CQueue::new();
        queue.append_tail(3);
        assert_eq!(queue.delete_head(), 3);
        assert_eq!(queue.delete_head(), -1);
    }

    #[test]
    fn test_2() {
        let mut queue = CQueue::new();
        assert_eq!(queue.delete_head(), -1);
        queue.append_tail(5);
        queue.append_tail(2);
        assert_eq!(queue.delete_head(), 5);
        assert_eq!(queue.delete_head(), 2);
    }
}
