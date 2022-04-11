class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    @classmethod
    def from_vals(cls, vals):
        head = ListNode(val=None)
        ref = head
        for val in vals:
            ref.next = ListNode(val=val)
            ref = ref.next
        return head.next

    def to_vals(self):
        vals = []
        while self is not None:
            vals.append(self.val)
            self = self.next
        return vals

    def display(self):
        vals = []
        ref = self
        while ref is not None:
            vals.append(ref.val)
            ref = ref.next
        return "->".join(str(val) for val in vals)


class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        front_ref, rear_ref = head, head
        for i in range(n):
            front_ref = front_ref.next

        while front_ref is not None and front_ref.next is not None:
            front_ref = front_ref.next
            rear_ref = rear_ref.next
        
        if front_ref is None:
            return rear_ref.next
        else:
            node_to_remove = rear_ref.next
            rear_ref.next = node_to_remove.next
            node_to_remove.next = None
            del node_to_remove

        return head