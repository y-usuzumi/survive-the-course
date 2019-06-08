from typing import List

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def mergeKLists(self, lists: List[ListNode]) -> ListNode:
        l = len(lists)
        if l == 0:
            return None
        if l == 1:
            return lists[0]
        if l == 2:
            l1, l2 = lists[0], lists[1]
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
        split_idx = l // 2
        return self.mergeKLists(
            [
                self.mergeKLists(lists[:split_idx]),
                self.mergeKLists(lists[split_idx:])
            ]
        )


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
    l1 = make_link_list(1, 4, 5)
    l2 = make_link_list(1, 3, 4)
    l3 = make_link_list(2, 6)
    print(display_link_list(l1))
    print(display_link_list(l2))
    print(display_link_list(l3))
    merged_list = sol.mergeKLists([l1, l2, l3])
    print(display_link_list(merged_list))
