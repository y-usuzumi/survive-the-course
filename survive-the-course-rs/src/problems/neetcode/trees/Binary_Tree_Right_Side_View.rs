// https://leetcode.com/problems/binary-tree-right-side-view/

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut curr_level = vec![root.unwrap()];

        while !curr_level.is_empty() {
            let last_node = curr_level.last().unwrap();
            result.push(last_node.borrow().val);
            let mut next_level = Vec::with_capacity(curr_level.len() * 2);
            for node in curr_level {
                let node = node.borrow();
                if let Some(ref node) = node.left {
                    next_level.push(node.clone());
                }

                if let Some(ref node) = node.right {
                    next_level.push(node.clone());
                }
            }
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
