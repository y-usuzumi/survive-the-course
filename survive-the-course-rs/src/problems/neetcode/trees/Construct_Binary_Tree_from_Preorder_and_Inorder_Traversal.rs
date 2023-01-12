// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.len() == 0 {
                return None;
            }
            let val = preorder[0];
            let idx_in_inorder = inorder.iter().position(|v| v == &val).unwrap();
            let left = helper(&preorder[1..idx_in_inorder + 1], &inorder[..idx_in_inorder]);
            let right = helper(
                &preorder[idx_in_inorder + 1..],
                &inorder[idx_in_inorder + 1..],
            );
            let mut node = TreeNode::new(val);
            node.left = left;
            node.right = right;
            return Some(Rc::new(RefCell::new(node)));
        }

        return helper(&preorder, &inorder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
