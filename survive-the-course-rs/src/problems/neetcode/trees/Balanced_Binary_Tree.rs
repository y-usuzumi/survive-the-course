// https://leetcode.com/problems/balanced-binary-tree/
pub struct Solution;

// Definition for a binary tree node.
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
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root).0
    }

    // Returns (is_balanced, height)
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            None => (true, 0),
            Some(node) => {
                let (bl, hl) = Self::helper(node.borrow().left.clone());
                let (br, hr) = Self::helper(node.borrow().right.clone());
                (bl && br && (hl - hr).abs() < 2, hl.max(hr) + 1)
            }
        }
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
