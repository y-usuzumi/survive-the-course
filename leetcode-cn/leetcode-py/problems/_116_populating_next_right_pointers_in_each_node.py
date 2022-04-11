class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
        if root is None:
            return

        nodes_to_connect = [root]

        while nodes_to_connect:
            for idx in range(0, len(nodes_to_connect) - 1):
                nodes_to_connect[idx].next = nodes_to_connect[idx+1]

            next_nodes_to_connect = []

            for node in nodes_to_connect:
                if node.left:
                    next_nodes_to_connect.append(node.left)
                    next_nodes_to_connect.append(node.right)

            nodes_to_connect = next_nodes_to_connect

        return root
