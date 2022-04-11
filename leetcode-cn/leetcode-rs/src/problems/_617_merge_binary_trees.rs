// <Place the leetcode link to the question here>

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }

        let root1_node = Rc::try_unwrap(root1.unwrap()).unwrap().into_inner();
        let root2_node = Rc::try_unwrap(root2.unwrap()).unwrap().into_inner();

        let mut new_root = TreeNode::new(root1_node.val + root2_node.val);
        let left = Self::merge_trees(root1_node.left, root2_node.left);
        let right = Self::merge_trees(root1_node.right, root2_node.right);
        new_root.left = left;
        new_root.right = right;
        Some(Rc::new(RefCell::new(new_root)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_1() {
        todo!("Assertion too cumbersome to implement");
    }
}
