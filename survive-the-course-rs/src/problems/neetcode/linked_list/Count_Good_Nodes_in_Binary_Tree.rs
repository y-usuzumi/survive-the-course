// https://leetcode.com/problems/count-good-nodes-in-binary-tree/

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::helper(root, i32::MIN);
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, curr_max: i32) -> i32 {
        if let Some(root) = root {
            let mut result = 0;
            let mut next_max = curr_max;
            let root_ref = root.borrow();
            if root_ref.val >= curr_max {
                result += 1;
                next_max = root_ref.val;
            }
            result += Self::helper(root_ref.left.clone(), next_max);
            result += Self::helper(root_ref.right.clone(), next_max);
            return result;
        }
        return 0;
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
