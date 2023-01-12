// https://leetcode.com/problems/binary-tree-maximum-path-sum/

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // (max_mergeable, max_unmergeable) Mergeable: a path that starts at
        // `root`. It can be further connected to its parent to form a longer
        // path. Unmergeable: a path whose root is not `root`. Either it does
        // not pass `root` or it spans across both left and right child.
        //
        // Current max mergeable = max(val, val+left_mergeable,
        // val+right_mergeable)
        //
        // Current max unmergeable = max(left_unmergeable, right_unmergeable,
        // val, val+left_mergeable, val+right_mergeable,
        // val+left_mergeable+right_mergeable)
        //
        // IMPROVEMENT: Current max unmergeable does not need to be returned.
        // We can use a global variable and update it in place.
        fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                None => (i32::MIN, i32::MIN),
                Some(node) => {
                    let noderef = node.borrow();
                    let (left_max_m, left_max_u) = helper(noderef.left.clone());
                    let (right_max_m, right_max_u) = helper(noderef.right.clone());
                    let mut max_m = noderef.val;
                    max_m += left_max_m.max(right_max_m).max(0);
                    let mut max_u = noderef.val;
                    if left_max_m > 0 {
                        max_u += left_max_m;
                    }
                    if right_max_m > 0 {
                        max_u += right_max_m;
                    }
                    max_u = max_u.max(left_max_u).max(right_max_u);
                    return (max_m, max_u);
                }
            }
        }

        return helper(root).1;
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
