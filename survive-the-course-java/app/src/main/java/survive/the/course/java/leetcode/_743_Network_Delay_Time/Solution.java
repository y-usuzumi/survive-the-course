package survive.the.course.java.leetcode._743_Network_Delay_Time;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;

class VertexDistPair {
    public int vertex;
    public int dist;
    public VertexDistPair(int vertex, int dist) {
        this.vertex = vertex;
        this.dist = dist;
    }
}

public class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        PriorityQueue<VertexDistPair> pq = new PriorityQueue<>((a, b) -> a.dist - b.dist);
        Map<Integer, List<VertexDistPair>> edges = new HashMap<>();
        Set<Integer> visited = new HashSet<>();
        int minTime = 0;
        for (int[] time : times) {
            edges.putIfAbsent(time[0], new ArrayList<>());
            List<VertexDistPair> dests = edges.get(time[0]);
            dests.add(new VertexDistPair(time[1], time[2]));
        }

        pq.add(new VertexDistPair(k, 0));

        while (!pq.isEmpty()) {
            VertexDistPair currVdp = pq.poll();
            if (visited.contains(currVdp.vertex)) {
                continue;
            }
            visited.add(currVdp.vertex);

            if (currVdp.dist > minTime) {
                minTime = currVdp.dist;
            }

            if (!edges.containsKey(currVdp.vertex)) {
                continue;
            }

            for (VertexDistPair vdp : edges.get(currVdp.vertex)) {
                if (visited.contains(vdp.vertex)) {
                    continue;
                }
                int newDist = currVdp.dist + vdp.dist;
                pq.add(new VertexDistPair(vdp.vertex, newDist));
            }
        }

        if (visited.size() != n) {
            return -1;
        }

        return minTime;
    }
}