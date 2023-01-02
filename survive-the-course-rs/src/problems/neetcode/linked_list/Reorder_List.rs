use std::{ptr::null, rc::Rc};

// https://leetcode.com/problems/reorder-list/
use crate::ds::linked_list::ListNode;

pub struct Solution;

impl Solution {
    // A painful mess of references, immutable references, unsafe and raw
    // pointers.
    //
    // I use `unsafe` block to get a mutable reference from an immutable one.
    // This is needed because we use the fast/slow pointer technique to find the
    // midpoint. The slow pointer cannot be mutable because we also need a
    // mutable reference to the head of the list when we do the merge.
    // Other solutions I see use one run of the list to find the length and
    // another run that stops at length/2. I don't like it because of the extra
    // run but it does save us from using unsafe block 🥲.
    pub fn reorder_list(step1: &mut Option<Box<ListNode>>) {
        // Basic cases
        if step1.is_none() || step1.as_ref().unwrap().next.is_none() {
            return;
        }

        let head = step1.take();

        // Two immutable pointers to find the midpoint
        let mut rslow = &head;
        let mut rfast = &head.as_ref().unwrap().next;
        // rslow moves 1 step each time; rfast moves 2 steps each time
        while !rfast.is_none() && !rfast.as_ref().unwrap().next.is_none() {
            rslow = &rslow.as_ref().unwrap().next;
            rfast = &rfast.as_ref().unwrap().next;
            rfast = &rfast.as_ref().unwrap().next;
        }

        // After the loop, rslow stops at the end of the first half of the list
        // (It can actually be one node longer than the second half. Doesn't
        // matter in this problem)

        // We already have a mutable reference to the head of list. What can I
        // do to get a mutable reference to the midpoint? Forgive me. I don't
        // know of any better solution.
        let rslow = unsafe {
            // Somehow I have to do:
            //     &Option<_> -> *const Option<_> -> *mut Option <_>
            (rslow as *const _ as *mut Option<Box<ListNode>>)
                .as_mut()
                .unwrap()
        };
        let l2 = rslow.as_mut().unwrap().next.take();
        let l2 = Self::reverse(l2);
        *step1 = Self::merge(head, l2);
    }

    fn reverse(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = l;
        while let Some(mut node) = curr {
            let next = node.next.take();
            curr = next;
            node.next = prev;
            prev = Some(node)
        }

        prev
    }

    fn merge(mut l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        let next = l1.as_mut().unwrap().next.take();
        l1.as_mut().unwrap().next = Self::merge(l2, next);
        l1
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
