// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    // The key takeaway here is if we have a mutable reference/mutable pointer to a node,
    // we can immediately replace the node. This means we do not need to keep the previous node
    // and update the next pointer.
    //
    // --------      --------
    // | val  |   -> | val  |
    // | next-|--/   | ...  |
    // --------      --------
    //                  ↑
    //                 &mut
    //
    // Other than that, just normal unsafe trick for two pointers in which one
    // is mutable.
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut left_node = &head;
        let mut right_node = &head;
        for _ in 0..n {
            if let Some(node) = right_node {
                right_node = &node.next
            } else {
                return head.unwrap().next;
            }
        }

        while let Some(node) = right_node {
            left_node = &left_node.as_ref().unwrap().next;
            right_node = &node.next
        }

        let left_node =
            unsafe { (left_node as *const _ as *mut Option<Box<ListNode>>).as_mut() }.unwrap();
        *left_node = left_node.as_mut().unwrap().next.take();

        head
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
