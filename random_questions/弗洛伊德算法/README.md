# 弗洛伊德算法

https://zh.wikipedia.org/wiki/Floyd-Warshall%E7%AE%97%E6%B3%95

解决任意两点间最短路径。可以正确处理有向图或负权（但不可存在负权回路）。一把就可以得出任意两点间的最短路径。

```
let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
for each vertex v
   dist[v][v] ← 0
for each edge (u,v)
   dist[u][v] ← w(u,v)  // the weight of the edge (u,v)
for k from 1 to |V|
   for i from 1 to |V|
      for j from 1 to |V|
         if dist[i][j] > dist[i][k] + dist[k][j] 
             dist[i][j] ← dist[i][k] + dist[k][j]
         end if
```
