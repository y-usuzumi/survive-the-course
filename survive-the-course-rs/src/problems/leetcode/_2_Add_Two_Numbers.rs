// https://leetcode.com/problems/add-two-numbers/

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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut curr_ref = &mut dummy;
        let mut carry = 0;
        let mut l1ref = l1.as_mut();
        let mut l2ref = l2.as_mut();
        loop {
            let mut sum = carry;
            let mut l_or_r = false;
            if let Some(l) = l1ref {
                sum += l.val;
                l1ref = l.next.as_mut();
                l_or_r = true;
            }
            if let Some(r) = l2ref {
                sum += r.val;
                l2ref = r.next.as_mut();
                l_or_r = true;
            }
            if l_or_r || sum > 0 {
                curr_ref.next = Some(Box::new(ListNode::new(sum % 10)));
                curr_ref = curr_ref.next.as_mut().unwrap();
                carry = sum / 10;
            } else {
                break;
            }
        }

        return dummy.next;
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
