// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/

use crate::ds::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut front_pointer = head.as_ref();
        let mut rear_pointer = head.as_mut();
        for _ in 0..n {
            if let Some(node) = front_pointer {
                front_pointer = node.next.as_ref();
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::remove_nth_from_end(), 0);
    }
}
