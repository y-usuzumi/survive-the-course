// https://leetcode.com/problems/insert-into-a-binary-search-tree/

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

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        let root = root.unwrap();
        let mut root_ref = root.borrow_mut();
        if val < root_ref.val {
            root_ref.left = Self::insert_into_bst(root_ref.left.clone(), val);
        } else {
            root_ref.right = Self::insert_into_bst(root_ref.right.clone(), val);
        }

        return Some(root.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
