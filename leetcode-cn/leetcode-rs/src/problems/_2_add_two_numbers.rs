pub struct Solution;

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

    fn from_vals(vals: Vec<i32>) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }
        let mut start_node = ListNode::new(vals[0]);
        let mut node = &mut start_node;
        for num in vals.iter().skip(1) {
            node.next = Some(Box::new(ListNode::new(*num)));
            node = node.next.as_mut().unwrap();
        }
        Some(Box::new(start_node))
    }

    fn to_vals(&self) -> Vec<i32> {
        let mut vals = vec![self.val];
        let mut node = self.next.as_ref();
        while let Some(n) = node {
            vals.push(n.val);
            node = n.next.as_ref();
        }
        vals
    }
}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut curr = &mut head;
        let mut carry = 0;
        let (mut lref1, mut lref2) = (l1.as_mut(), l2.as_mut());
        loop {
            if !lref1.is_some() || !lref2.is_some() {
                break;
            }
            let b1 = lref1.unwrap();
            let b2 = lref2.unwrap();
            let sum = b1.val + b2.val + carry;
            carry = if sum > 9 { 1 } else { 0 };
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            lref1 = b1.next.as_mut();
            lref2 = b2.next.as_mut();
        }

        let mut remaining;
        if lref1.is_none() {
            remaining = lref2;
        } else {
            remaining = lref1;
        }

        while let Some(b) = remaining {
            let sum = b.val + carry;
            carry = if sum > 9 { 1 } else { 0 };
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            remaining = b.next.as_mut();
        }
        if carry == 1 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let ln = ListNode::from_vals(vec![1, 2, 3]).unwrap();
        let v = ln.to_vals();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_1() {
        let l1 = ListNode::from_vals(vec![2, 4, 3]);
        let l2 = ListNode::from_vals(vec![5, 6, 4]);
        assert_eq!(
            Solution::add_two_numbers(l1, l2).unwrap().to_vals(),
            vec![7, 0, 8]
        )
    }
}
