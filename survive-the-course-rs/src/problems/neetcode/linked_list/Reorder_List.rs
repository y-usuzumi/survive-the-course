use std::rc::Rc;

// https://leetcode.com/problems/reorder-list/
use crate::ds::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(step1: &mut Option<Box<ListNode>>) {
        todo!();
        // let head_rc = Rc::new(step1);
        // let mut step2 = &*step1;
        // let mut old_head = step1;
        // while !step2.is_none() && !step2.as_ref().unwrap().next.is_none() {
        //     step1 = &mut step1.as_ref().unwrap().next;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        todo!();
        //assert_eq!(Solution::function(), 0);
    }
}
