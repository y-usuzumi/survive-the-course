// https://leetcode.com/problems/count-pairs-of-nodes/

package survive.the.course.java.leetcode._1782_Count_Pairs_of_Nodes;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;

class Solution {
    // Idea:
    // 1. Create a list of the degrees of all nodes.
    // 2. Sort the list.
    // 3. Use double pointer to count the number of node pairs that satisfy
    //    degrees[i] + degrees[j] > query
    // 4. Iterate over edges once again to find all node pairs that satisfy
    //    degrees[i] + degrees[j] - edges[i,j] <= query
    // 5. Exclude the number of node pairs from step 4 from the count of step 3.
    public int[] countPairs(int n, int[][] edges, int[] queries) {
        int[] nodeDegrees = new int[n];
        HashMap<Integer, Integer> edgeMap = new HashMap<>();
        Set<Integer> distinctEdges = new HashSet<>();

        for (int[] edge : edges) {
            int a = edge[0];
            int b = edge[1];

            int key = encodePair(a, b);
            edgeMap.put(key, edgeMap.getOrDefault(key, 0) + 1);
            nodeDegrees[a-1] += 1;
            nodeDegrees[b-1] += 1;
            distinctEdges.add(key);
        }

        int[] sortedNodeDegrees = nodeDegrees.clone();
        Arrays.sort(sortedNodeDegrees);

        int[] results = new int[queries.length];
        for (int queryIdx = 0; queryIdx < queries.length; queryIdx++) {
            int query = queries[queryIdx];
            int left = 0;
            int right = sortedNodeDegrees.length - 1;
            int count = 0;
            while (left < right) {
                if (sortedNodeDegrees[left] + sortedNodeDegrees[right] <= query) {
                    left++;
                    continue;
                }
                count += right - left;
                right--;
            }
            // Make sure to use de-duped edges. Otherwise we will subtract from
            // count more than needed.
            for (int edge : distinctEdges) {
                int source = edge >> 15;
                int dest = edge % (1 << 15);
                int sumOfDegrees = nodeDegrees[source-1] + nodeDegrees[dest-1];
                int incident = sumOfDegrees - edgeMap.getOrDefault(encodePair(source, dest), 0);
                if (sumOfDegrees > query && incident <= query) {
                    count -= 1;
                }
            }

            results[queryIdx] = count;
        }

        return results;
    }

    private int encodePair(int a, int b) {
        if (a > b) {
            a = a ^ b;
            b = a ^ b;
            a = a ^ b;
        }
        return a << 15 | b;
    }
}
