// https://leetcode-cn.com/problems/middle-of-the-linked-list/

use crate::ds::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut step1 = head.as_ref();
        let mut step2 = head.as_ref();
        while step2.is_some() {
            step2 = step2.unwrap().next.as_ref();
            if step2.is_none() {
                break;
            } else {
                step1 = step1.unwrap().next.as_ref();
                step2 = step2.unwrap().next.as_ref();
            }
        }
        step1.cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::middle_node(ListNode::from_vals(&[1, 2, 3, 4, 5]))
                .unwrap()
                .to_vals(),
            vec![3, 4, 5]
        );
    }
}
