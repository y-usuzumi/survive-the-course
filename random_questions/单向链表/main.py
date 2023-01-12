class Node:
    def __init__(self, n, next_node=None):
        self._n = n
        self._next_node = next_node

    @property
    def value(self):
        return self._n

    @property
    def next_node(self):
        return self._next_node

    def __str__(self):
        if self._next_node is None:
            return "(%s)" % self._n
        else:
            return "(%s) -> %s" % (self._n, self._next_node)


def linked_list_from_list(l):
    # [1, 4, 7, 2, 9, 5, 3]
    # 1 -> 4 -> 7 -> 2 -> 9 -> 5 -> 3
    node = None
    for i in reversed(l):
        node = Node(i, next_node=node)
    return node


if __name__ == '__main__':
    l = [1, 4, 7, 2, 9, 5, 3]
    ll = linked_list_from_list(l)
    print(ll)

    print("Head is: %s" % ll.value)
    print("Tail is: %s" % ll.next_node)
