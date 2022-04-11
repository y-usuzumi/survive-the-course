// https://leetcode-cn.com/problems/merge-two-sorted-lists/

use crate::ds::linked_list::ListNode;
use std::mem::replace;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr_node_1 = list1;
        let mut curr_node_2 = list2;
        let mut merge_head = ListNode::new(0);
        let mut merge_node = &mut merge_head;
        loop {
            if curr_node_1.is_none() {
                merge_node.next = curr_node_2;
                break;
            }

            if curr_node_2.is_none() {
                merge_node.next = curr_node_1;
                break;
            }

            let mut curr_node_1_unwrapped = curr_node_1.as_mut().unwrap().to_owned();
            let mut curr_node_2_unwrapped = curr_node_2.as_mut().unwrap().to_owned();
            if curr_node_1_unwrapped.val < curr_node_2_unwrapped.val {
                curr_node_1 = replace(&mut curr_node_1_unwrapped.next, None);
                merge_node.next = Some(curr_node_1_unwrapped);
                merge_node = merge_node.next.as_mut().unwrap();
            } else {
                curr_node_2 = replace(&mut curr_node_2_unwrapped.next, None);
                merge_node.next = Some(curr_node_2_unwrapped);
                merge_node = merge_node.next.as_mut().unwrap();
            }
        }

        merge_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let left = ListNode::from_vals(&[1, 2, 4]);
        let right = ListNode::from_vals(&[1, 3, 4]);
        assert_eq!(
            Solution::merge_two_lists(left, right).unwrap().to_vals(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn test_2() {
        let left = ListNode::from_vals(&[]);
        let right = ListNode::from_vals(&[]);
        assert_eq!(Solution::merge_two_lists(left, right), None);
    }
}
