import heapq


def max_series(input_series, window_size):
    # 除了数值之外，保存数值对应的位置。
    # 数值位置用于判断当前最大值是否超出了窗口范围。
    heap = [(-i, idx) for idx, i in enumerate(input_series[:window_size])]
    # O(n) 构建堆
    heapq.heapify(heap)

    # 第一个最大值。即便input_series长度 < window_size，也应该至少输出一个最大值。
    yield -heap[0][0]
    for idx in range(window_size, len(input_series)):
        top = heap[0]
        # 如果当前最大值的位置超出了窗口范围，先把它踢出去。
        if top[1] <= idx - window_size:
            heapq.heappop(heap)
        # 把新值加进堆里。
        heapq.heappush(heap, (-input_series[idx], idx))
        # 避免堆过大，偶尔清理一下。
        # 如果是用二叉查找树，则可以随时保持树中节点个数恒定，而不需要做清理操作，
        # 相应地牺牲了插入和查找性能。
        if len(heap) > 10 * window_size:
            heap = heapq.nlargest(window_size, heap)
        yield -heap[0][0]


if __name__ == '__main__':
    g = max_series([3, 1, 2, 6, 9, 2, 7, 4], 3)
    print(list(g))
