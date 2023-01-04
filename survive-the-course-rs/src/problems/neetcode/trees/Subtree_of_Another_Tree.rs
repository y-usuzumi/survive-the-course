// https://leetcode.com/problems/subtree-of-another-tree/

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
    // Someone actually cheated by using the derived Eq implementation i.e.
    // if treenode1 == treenode2, then they are exactly the same.
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root.clone(), sub_root) {
            (_, None) => true,
            (None, _) => false,
            (Some(p), sub_root) => {
                let rp = p.borrow();
                Self::is_equal(root, sub_root.clone())
                    || Self::is_subtree(rp.left.clone(), sub_root.clone())
                    || Self::is_subtree(rp.right.clone(), sub_root)
            }
        }
    }

    fn is_equal(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let rl = l.borrow();
                let rr = r.borrow();
                rl.val == rr.val
                    && Self::is_equal(rl.left.clone(), rr.left.clone())
                    && Self::is_equal(rl.right.clone(), rr.right.clone())
            }
            _ => false,
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
