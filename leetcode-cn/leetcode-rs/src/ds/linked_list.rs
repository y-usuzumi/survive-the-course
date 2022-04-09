// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vals(vals: Vec<i32>) -> Self {
        let mut head = Self::new(0);
        let mut pointer = &mut head;
        for val in vals {
            let next_node = Self::new(val);
            pointer.next = Some(Box::new(next_node));
            pointer = pointer.next.as_mut().unwrap()
        }
        *head.next.unwrap()
    }

    pub fn to_vals(&self) -> Vec<i32> {
        let mut res = Vec::new();
        let mut pointer = self;
        loop {
            res.push(pointer.val);
            if pointer.next.is_none() {
                break;
            }
            pointer = pointer.next.as_ref().unwrap();
        }
        res
    }
}
