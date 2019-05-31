package ratina;

import java.util.ArrayList;

/**
 * Hello world!
 *
 */

class Tree<T> {
    Node<T> root;
    public Tree(Node<T> root) {
        this.root = root;
    }
}

class Node<T> {
    Node<T> left, right;
    T value;

    public Node(T value) {
        this.value = value;
    }

    public Node(T value, Node<T> left, Node<T> right) {
        this.value = value;
        this.left = left;
        this.right = right;
    }

    public T getValue() {
        return value;
    }

}
public class App
{
    static ArrayList<ArrayList<Integer>> findPaths(Tree<Integer> tree, int sum) {
        if (tree.root == null) {
            return new ArrayList<>();
        }
        return _findPaths(tree.root, sum, sum, new ArrayList<Integer>(), false);
    }

    static ArrayList<ArrayList<Integer>> _findPaths(Node<Integer> node, int origVal, int currVal, ArrayList<Integer> currPath, boolean nobranch) {
        if (node == null) {
            return new ArrayList<>();
        }
        currVal = currVal - node.value;
        ArrayList<ArrayList<Integer>> result = new ArrayList<>();
        ArrayList<Integer> newPath = (ArrayList<Integer>)currPath.clone();
        newPath.add(node.value);

        if (currVal == 0) {
            result.add(newPath);
        }

        if (node.left != null) {
            result.addAll(_findPaths(node.left, origVal, currVal, newPath, true));
            if (!nobranch) {
                result.addAll(_findPaths(node.left, origVal, origVal, new ArrayList<Integer>(), false));
            }

        }

        if (node.right != null) {
            result.addAll(_findPaths(node.right, origVal, currVal, newPath, true));
            if (!nobranch) {
                result.addAll(_findPaths(node.right, origVal, origVal, new ArrayList<Integer>(), false));
            }
        }

        return result;
    }
    public static void main( String[] args )
    {
        Tree<Integer> t = new Tree(
            new Node(
                2,
                new Node(
                    3,
                    null,
                    new Node(4)
                    ),
                new Node(
                    5,
                    new Node(
                        1,
                        new Node(1),
                        new Node(
                            0,
                            null,
                            new Node(
                                1,
                                new Node(7),
                                null
                                )
                            )
                        ),
                    new Node(2)
                    )
                )
        );
        System.out.println(findPaths(t, 9));
    }
}
