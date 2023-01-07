// https://leetcode.com/problems/binary-tree-level-order-traversal/

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }
        let mut curr_level = vec![root.unwrap()];
        while !curr_level.is_empty() {
            let mut curr_level_vals = Vec::with_capacity(curr_level.len());

            let mut next_level = Vec::with_capacity(curr_level.len() * 2);
            for node in curr_level {
                let node_ref = node.borrow();
                curr_level_vals.push(node_ref.val);
                if let Some(ref node) = node_ref.left {
                    next_level.push(node.clone());
                }
                if let Some(ref node) = node_ref.right {
                    next_level.push(node.clone());
                }
            }

            result.push(curr_level_vals);
            curr_level = next_level;
        }

        result
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
