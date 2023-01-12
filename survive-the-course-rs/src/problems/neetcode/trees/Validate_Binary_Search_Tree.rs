// https://leetcode.com/problems/validate-binary-search-tree/

pub struct Solution;

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::helper(root, i32::MAX, i32::MIN);
    }

    // The test cases contain corner cases including i32::MIN && i32::MAX. We
    // can use i64 but we are only dodging the challenge other than confronting
    // it. We can perfectly use i32 and take the edge cases into special
    // consideration.
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, lbound: i32, rbound: i32) -> bool {
        match root {
            None => true,
            Some(node) => {
                let noderef = node.borrow();
                let val = noderef.val;
                // We use i32::MIN to denote unbounded min value and i32::MAX for
                // unbounded max value.
                if lbound != i32::MAX && val <= lbound || rbound != i32::MIN && val >= rbound {
                    return false;
                }

                // If the current value is i32::MIN or i32::MAX, its children
                // need to be bounded. If the value is i32::MAX and has a right
                // child, we should not recurse into its child as we will not be
                // able to set a min bound. Therefore we do some special check
                if val == i32::MAX && noderef.right.is_some()
                    || val == i32::MIN && noderef.left.is_some()
                {
                    return false;
                }

                return Self::helper(noderef.left.clone(), lbound, val)
                    && Self::helper(noderef.right.clone(), val, rbound);
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
