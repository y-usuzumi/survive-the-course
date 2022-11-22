// https://leetcode.com/problems/merge-two-sorted-lists/

use crate::ds::linked_list::ListNode;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut prev = &mut result;
        // Not able to use while let binding due to https://github.com/rust-lang/rust/issues/92858
        while list1.is_some() && list2.is_some() {
            let mut node1 = list1.unwrap();
            let mut node2 = list2.unwrap();

            if node1.val < node2.val {
                let node1next = node1.next;
                node1.next = None;
                prev.next = Some(node1);
                list1 = node1next;
                list2 = Some(node2);
                prev = prev.next.as_mut().unwrap();
            } else {
                let node2next = node2.next;
                node2.next = None;
                prev.next = Some(node2);
                list2 = node2next;
                list1 = Some(node1);
                prev = prev.next.as_mut().unwrap();
            }
        }

        if list1.is_some() {
            prev.next = list1;
        }

        if list2.is_some() {
            prev.next = list2;
        }
        return result.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let l1 = ListNode::from_vals(&[1, 2, 4]);
        let l2 = ListNode::from_vals(&[1, 3, 4]);
        assert_eq!(
            Solution::merge_two_lists(l1, l2),
            ListNode::from_vals(&[1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn test_2() {
        let l1 = ListNode::from_vals(&[]);
        let l2 = ListNode::from_vals(&[]);
        assert_eq!(Solution::merge_two_lists(l1, l2), ListNode::from_vals(&[]));
    }

    #[test]
    fn test_3() {
        let l1 = ListNode::from_vals(&[]);
        let l2 = ListNode::from_vals(&[0]);
        assert_eq!(Solution::merge_two_lists(l1, l2), ListNode::from_vals(&[0]));
    }
}
