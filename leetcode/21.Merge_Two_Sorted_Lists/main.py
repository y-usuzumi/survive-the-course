# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        start = curr = ListNode(0)
        while l1 != None and l2 != None:
            if l1.val <= l2.val:
                curr.next = l1
                l1 = l1.next
            else:
                curr.next = l2
                l2 = l2.next
            curr = curr.next
        if l1 != None:
            curr.next = l1
        else:
            curr.next = l2
        return start.next

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
    l1 = make_link_list(1, 2, 4)
    l2 = make_link_list(1, 3, 4)
    print(display_link_list(l1))
    print(display_link_list(l2))
    print(display_link_list(sol.mergeTwoLists(l1, l2)))  # 1 -> 1 -> 2 -> 3 -> 4 -> 4
