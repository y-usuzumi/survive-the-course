class Tree:
    __slots__ = ['root']

    def __init__(self, root):
        self.root = root


class Node:
    __slots__ = ['value', 'left', 'right']

    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left, self.right = left, right

    def __repr__(self):
        return "Node(%d)" % self.value


def find_paths(t, val):
    if t.root is None:
        return []

    return _find_paths(t.root, val, val, [], False)


def _find_paths(root, original_val, val, curr_path, nobranch):
    if root is None:
        return []
    val = val - root.value
    result = []
    if val == 0:
        result.append(curr_path + [root])
    if root.left is not None:
        result.extend(_find_paths(root.left, original_val, val, curr_path + [root], True))
        if not nobranch:
            result.extend(_find_paths(root.left, original_val, original_val, [], False))
    if root.right is not None:
        result.extend(_find_paths(root.right, original_val, val, curr_path + [root], True))
        if not nobranch:
            result.extend(_find_paths(root.right, original_val, original_val, [], False))

    return result


if __name__ == '__main__':
    tree = Tree(
        Node(
            2,
            left=Node(
                3,
                right=Node(4)
            ),
            right=Node(
                5,
                left=Node(
                    1,
                    left=Node(1),
                    right=Node(
                        0,
                        right=Node(
                            1,
                            left=Node(7)
                        ),
                    )
                ),
                right=Node(2)
            )
        )
    )
    for p in find_paths(tree, 9):
        print([n.value for n in p])
