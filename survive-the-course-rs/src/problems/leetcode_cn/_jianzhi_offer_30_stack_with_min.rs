// https://leetcode.cn/problems/bao-han-minhan-shu-de-zhan-lcof/

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { stack: Vec::new(), min_stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_stack.is_empty() {
            self.min_stack.push(x);
        } else {
            self.min_stack.push(x.min(*self.min_stack.last().unwrap()));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().cloned().unwrap_or(-1)
    }

    fn min(&self) -> i32 {
        self.min_stack.last().cloned().unwrap_or(-1)
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.min(), -2);
    }
}