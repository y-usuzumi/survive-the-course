// https://leetcode.com/problems/diameter-of-binary-tree/

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::helper(root).0 - 1;
    }

    // Returns (max_diameter, max_depth)
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => {
                let (diameter_l, max_depth_l) = Self::helper(node.borrow().left.clone());
                let (diameter_r, max_depth_r) = Self::helper(node.borrow().right.clone());

                let max_depth = max_depth_l.max(max_depth_r) + 1;
                let diameter = diameter_l.max(diameter_r).max(max_depth_l + max_depth_r);
                (diameter, max_depth)
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
