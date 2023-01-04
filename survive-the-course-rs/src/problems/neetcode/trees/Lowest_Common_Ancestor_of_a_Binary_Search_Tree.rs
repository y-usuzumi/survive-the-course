// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            None => None,
            Some(node) => {
                let rn = node.borrow();
                let rcp = p.clone().unwrap();
                let rcq = q.clone().unwrap();
                let rp = rcp.borrow();
                let rq = rcq.borrow();
                if rp.val == rn.val || rq.val == rn.val {
                    return root;
                }

                if rp.val < rn.val && rq.val < rn.val {
                    Self::lowest_common_ancestor(rn.left.clone(), p, q)
                } else if rp.val > rn.val && rq.val > rn.val {
                    Self::lowest_common_ancestor(rn.right.clone(), p, q)
                } else {
                    root
                }
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
