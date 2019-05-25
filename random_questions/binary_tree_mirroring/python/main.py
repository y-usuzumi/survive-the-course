from tree import *

if __name__ == '__main__':
    root_node = Node(
        5,
        left=Node(3, left=Node(1, right=Node(2))),
        right=Node(7, left=Node(6), right=Node(8))
    )
    tree = Tree(root_node)

    print(tree)
    print(tree.mirror())
