package survive.the.course.java.leetcode._133_Clone_Graph;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

// Definition for a Node.
class Node {
    public int val;
    public List<Node> neighbors;

    public Node() {
        val = 0;
        neighbors = new ArrayList<Node>();
    }

    public Node(int _val) {
        val = _val;
        neighbors = new ArrayList<Node>();
    }

    public Node(int _val, ArrayList<Node> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
}

class Solution {
    public Node cloneGraph(Node node) {
        if (node == null) {
            return null;
        }
        Map<Integer, Node> nodes = new HashMap<>();
        ArrayDeque<Node> q = new ArrayDeque<>();
        q.add(node);
        Node result = new Node(node.val);
        nodes.put(node.val, result);
        while (!q.isEmpty()) {
            Node oldNode = q.pollFirst();
            Node newNode = nodes.get(oldNode.val);
            for (Node neighbor : oldNode.neighbors) {
                Node newNeighbor;
                if (nodes.containsKey(neighbor.val)) {
                    newNeighbor = nodes.get(neighbor.val);
                } else {
                    newNeighbor = new Node(neighbor.val);
                    nodes.put(neighbor.val, newNeighbor);
                    q.addLast(neighbor);
                }
                newNode.neighbors.add(newNeighbor);
            }
        }
        return result;
    }
}