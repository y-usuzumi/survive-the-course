// https://leetcode.com/problems/reverse-linked-list/

use crate::ds::linked_list::ListNode;

pub trait Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

pub struct SolutionSimple;
pub struct SolutionInPlace;

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

impl SolutionSimple {
    fn reverse_helper(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        while !left.is_none() {
            let left_unwrapped = left.unwrap();
            let mut node = ListNode::new(left_unwrapped.val);
            node.next = right;
            left = left_unwrapped.next;
            right = Some(Box::new(node));
        }
        return right;
    }
}

impl Solution for SolutionSimple {
    // Same idea as https://hackage.haskell.org/package/base-4.17.0.0/docs/src/GHC.List.html#reverse
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_helper(head, None)
    }
}

impl Solution for SolutionInPlace {
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            // TODO: Read more about partial move!
            // This is pretty interesting. Moving the `next` field of node results
            // in a partial move. In the next line, by reassigning a value to the
            // moved field, the full ownership is recovered.
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            head = next;
        }
        return prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod solution_simple {
        use super::*;
        #[test]
        fn test_1() {
            let list_node = ListNode::from_vals(&[1, 2]);
            assert_eq!(
                SolutionSimple::reverse_list(list_node),
                ListNode::from_vals(&[2, 1])
            );
        }

        #[test]
        fn test_2() {
            let list_node = ListNode::from_vals(&[]);
            assert_eq!(
                SolutionSimple::reverse_list(list_node),
                ListNode::from_vals(&[])
            );
        }
    }

    mod solution_in_place {
        use super::*;
        #[test]
        fn test_1() {
            let list_node = ListNode::from_vals(&[1, 2]);
            assert_eq!(
                SolutionInPlace::reverse_list(list_node),
                ListNode::from_vals(&[2, 1])
            );
        }

        #[test]
        fn test_2() {
            let list_node = ListNode::from_vals(&[]);
            assert_eq!(
                SolutionInPlace::reverse_list(list_node),
                ListNode::from_vals(&[])
            );
        }
    }
}
