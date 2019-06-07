# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        init = result = ListNode(0)
        carry = 0
        while l1 is not None and l2 is not None:
            s = l1.val + l2.val + carry
            n = s % 10
            carry = s // 10
            result.next = ListNode(n)
            result = result.next
            l1, l2 = l1.next, l2.next
        if l1 is None:
            while l2 is not None:
                s = l2.val + carry
                n = s % 10
                carry = s // 10
                result.next = ListNode(n)
                result = result.next
                l2 = l2.next
        if l2 is None:
            while l1 is not None:
                s = l1.val + carry
                n = s % 10
                carry = s // 10
                result.next = ListNode(n)
                result = result.next
                l1 = l1.next
        if carry:
            result.next = ListNode(1)
        return init.next
