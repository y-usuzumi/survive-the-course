class Graph(object):
    def __init__(self):
        self._vertices = []
        self._adjacency_list = []

    def insert(self, vertex):
        self._vertices.append(vertex)
        self._adjacency_list.append([])

    def connect(self, a, b):
        self._adjacency_list[a].append(b)
        self._adjacency_list[b].append(a)

    @property
    def vertices(self):
        return self._vertices

    def _traverse_dfs(self, vertex_idx, visit_state):
        if visit_state[vertex_idx]:
            return
        visit_state[vertex_idx] = True
        yield (vertex_idx, self._vertices[vertex_idx])
        peers = self._adjacency_list[vertex_idx]
        for peer in peers:
            yield from self._traverse_dfs(peer, visit_state)

    def traverse_dfs(self):
        visit_state = [False] * len(self._vertices)
        for idx, _ in enumerate(self._vertices):
            if visit_state[idx]:
                continue
            yield from self._traverse_dfs(idx, visit_state)

    def traverse_from(self, vertex_idx):
        visit_state = [False] * len(self._vertices)
        return self._traverse_dfs(vertex_idx, visit_state)

    def is_reachable(self, vertex_idx_a, vertex_idx_b):
        for idx, _ in self.traverse_from(vertex_idx_a):
            if idx == vertex_idx_b:
                return True
        return False


def input_pair():
    a, b = input().split(" ")
    return int(a), int(b)


def is_reachable(a, b, jump_dist):
    x1, y1 = a
    x2, y2 = b
    dist = (x1-x2) * (x1-x2) + (y1-y2) * (y1-y2)
    jd2 = jump_dist * 2
    return jd2 > dist


def is_reachable_to_edge(vertex, jump_dist):
    x, y = vertex
    return (
        x <= -50 + jump_dist
        or x >= 50 - jump_dist
        or y <= -50 + jump_dist
        or y >= 50 - jump_dist
    )


def is_reachable_to_land(vertex, jump_dist):
    x, y = vertex
    radius = (x*x + y*y)**(0.5)
    return (radius - 15) < jump_dist


def print_result(ret):
    print("Yes" if ret else "No")


if __name__ == '__main__':
    croc_cnt, jump_dist = input_pair()
    g = Graph()

    # The first vertex is James
    g.insert("James")
    for _ in range(croc_cnt):
        g.insert(input_pair())

    # The last vertex is the edge
    g.insert("EDGE")

    vertices = g.vertices
    for idx0 in range(1, croc_cnt+1):
        for idx1 in range(idx0+1, croc_cnt+1):
            if is_reachable(vertices[idx0], vertices[idx1], jump_dist):
                g.connect(idx0, idx1)

        vertex = vertices[idx0]
        # Check for connectivity between crocodile and edge
        if is_reachable_to_edge(vertex, jump_dist):
            g.connect(idx0, len(vertices) - 1)
        if is_reachable_to_land(vertex, jump_dist):
            g.connect(idx0, 0)

    print_result(g.is_reachable(0, len(vertices) - 1))
