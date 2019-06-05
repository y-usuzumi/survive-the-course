# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        last = first = head
        for _ in range(n):
            first = first.next
        if first is None:
            return last.next
        while first.next is not None:
            first = first.next
            last = last.next
        last.next = last.next.next
        return head


def make_link_list(*vals):
    if not vals:
        return None
    start = curr = ListNode(vals[0])
    for idx in range(1, len(vals)):
        curr.next = ListNode(vals[idx])
        curr = curr.next
    return start


def display_link_list(node):
    from io import StringIO
    vals = []
    while node is not None:
        vals.append(node.val)
        node = node.next
    return " -> ".join([str(val) for val in vals])


if __name__ == '__main__':
    sol = Solution()
    l1 = make_link_list(1, 2, 3, 4, 5)
    print(display_link_list(sol.removeNthFromEnd(l1, 2)))
    l1 = make_link_list(1)
    print(display_link_list(sol.removeNthFromEnd(l1, 1)))
    l1 = make_link_list(1, 2)
    print(display_link_list(sol.removeNthFromEnd(l1, 2)))

