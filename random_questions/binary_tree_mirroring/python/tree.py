class Node:
    __slots__ = ['left', 'right', 'value']

    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right

    def __str__(self):
        return "<%s>(left=%s, right=%s)" % (
            self.value,
            str(self.left),
            str(self.right)
        )

    def mirror(self):
        right_mirror = self.right and self.right.mirror()
        left_mirror = self.left and self.left.mirror()

        return Node(value=self.value, left=right_mirror, right=left_mirror)


class Tree:
    __slots__ = ['root']

    def __init__(self, root=None):
        self.root = root

    @property
    def is_empty(self):
        return self.root is None

    def __str__(self):
        return "Tree(%s)" % str(self.root)

    def mirror(self):
        return Tree(self.root.mirror())
