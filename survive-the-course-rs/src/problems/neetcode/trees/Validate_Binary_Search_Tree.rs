// https://leetcode.com/problems/validate-binary-search-tree/

pub trait Solution {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

pub struct Solution1;
pub struct Solution2;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution for Solution1 {
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // The test cases contain corner cases including i32::MIN && i32::MAX. We
        // can use i64 but we are only dodging the challenge other than confronting
        // it. We can perfectly use i32 and take the edge cases into special
        // consideration.
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, lbound: i32, rbound: i32) -> bool {
            match root {
                None => true,
                Some(node) => {
                    let noderef = node.borrow();
                    let val = noderef.val;
                    // We use i32::MIN to denote unbounded min value and i32::MAX for
                    // unbounded max value.
                    if lbound != i32::MAX && val <= lbound || rbound != i32::MIN && val >= rbound {
                        return false;
                    }

                    // If the current value is i32::MIN or i32::MAX, its children
                    // need to be bounded. If the value is i32::MAX and has a right
                    // child, we should not recurse into its child as we will not be
                    // able to set a min bound. Therefore we do some special check
                    if val == i32::MAX && noderef.right.is_some()
                        || val == i32::MIN && noderef.left.is_some()
                    {
                        return false;
                    }

                    return helper(noderef.left.clone(), lbound, val)
                        && helper(noderef.right.clone(), val, rbound);
                }
            }
        }
        return helper(root, i32::MAX, i32::MIN);
    }
}

impl Solution for Solution2 {
    // In-order traversal produces a monotonic increasing list. When we pop a
    // value from the stack, we compare it with the previous value.
    fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut curr = root;
        let mut stack = vec![];
        let mut prev = None;

        loop {
            while let Some(node) = curr {
                {
                    let noderef = node.borrow();
                    curr = noderef.left.clone();
                }
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                let noderef = node.borrow();
                if prev.map_or(false, |prev| noderef.val <= prev) {
                    return false;
                }
                prev = Some(noderef.val);
                curr = noderef.right.clone();
                continue;
            }
            break;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
    }
}
