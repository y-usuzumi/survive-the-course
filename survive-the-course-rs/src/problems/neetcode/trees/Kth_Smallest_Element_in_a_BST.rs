// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

pub trait Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32;
}

pub struct Solution1;
pub struct Solution2;

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

impl Solution for Solution1 {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, k: usize) -> (usize, i32) {
            match root {
                None => (k, -1),
                Some(node) => {
                    let node = node.borrow();
                    let (remaining, found) = helper(node.left.clone(), k);
                    if found >= 0 {
                        return (remaining, found);
                    }

                    if remaining == 1 {
                        return (0, node.val);
                    }

                    return helper(node.right.clone(), remaining - 1);
                }
            }
        }

        let (_, result) = helper(root, k as usize);
        return result;
    }
}

impl Solution for Solution2 {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut curr = root;
        let mut curr_k = 0;
        loop {
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                curr_k += 1;
                if curr_k == k {
                    return node.borrow().val;
                }
                curr = node.borrow().right.clone();
                continue;
            }
            break;
        }

        return -1;
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
