import numpy as np

class Graph(object):
    '''Graph

    edges:
    a list of (from, to, weight) tuples where from and to are positions other than values
    '''
    def __init__(self, vertices, edges):
        self._vertices = vertices
        self._lrmap = {}
        self._rlmap = {}
        for f, t, w in edges:
            rmap = self._lrmap.setdefault(f, {})
            rmap[t] = w
            lmap = self._rlmap.setdefault(t, {})
            lmap[f] = w

    def floyd(self):
        table = []
        i = 0
        cnt = len(self._vertices)
        while i < cnt:
            row = [2 ** 31 - 1] * cnt
            table.append(row)
            i += 1
        for l, rmap in self._lrmap.items():
            for r, w in rmap.items():
                table[l][r] = w
                table[r][l] = w
        i = 0
        while i < cnt:
            table[i][i] = 0
            i += 1

        k = 0
        while k < cnt:
            i = 0
            while i < cnt:
                j = 0
                while j < cnt:
                    dist = table[i][k] + table[k][j]
                    if dist < table[i][j]:
                        table[i][j] = dist
                    j += 1
                i += 1
            k += 1
        table = np.array([np.array(arr) for arr in table])
        print(table)


if __name__ == '__main__':
    pass
