// https://leetcode.com/problems/delete-node-in-a-bst/

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

pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        fn pop_min(
            root: Rc<RefCell<TreeNode>>,
        ) -> (Rc<RefCell<TreeNode>>, Option<Rc<RefCell<TreeNode>>>) {
            let mut root_ref = root.borrow_mut();
            if root_ref.left.is_none() {
                return (root.clone(), root_ref.right.clone());
            }

            let (min, remaining) = pop_min(root_ref.left.clone().unwrap());
            root_ref.left = remaining;
            return (min, Some(root.clone()));
        }

        let root = root.unwrap();
        let mut root_ref = root.borrow_mut();
        if root_ref.val < key {
            root_ref.right = Self::delete_node(root_ref.right.clone(), key);
            return Some(root.clone());
        } else if root_ref.val > key {
            root_ref.left = Self::delete_node(root_ref.left.clone(), key);
            return Some(root.clone());
        } else if root_ref.right.is_none() {
            return root_ref.left.clone();
        } else if root_ref.left.is_none() {
            return root_ref.right.clone();
        } else {
            let (new_root, remaining) = pop_min(root_ref.right.clone().unwrap());
            let mut new_root_ref = new_root.borrow_mut();
            new_root_ref.right = remaining;
            new_root_ref.left = root_ref.left.clone();
            return Some(new_root.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
