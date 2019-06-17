class _Value:
    __slots__ = ['key', 'val', 'prev_val', 'next_val']

    def __init__(self, key, val):
        self.key = key
        self.val = val
        self.next_val = None
        self.prev_val = None

    def __repr__(self):
        return f"{{{self.prev_val.val if self.prev_val is not None else None} <- {self.val} -> {self.next_val.val if self.next_val is not None else None}}}"


class LRUCache:

    def __init__(self, capacity: int):
        self._cap = capacity
        self._cache = {}
        self._head = None
        self._last = None
        self._curr_cnt = 0

    def _get(self, key: int) -> _Value:
        val = self._cache.get(key)
        if val is None:
            return None
        if val.prev_val is not None:
            if self._last is val:
                self._last = val.prev_val
            val.prev_val.next_val = val.next_val
            if val.next_val is not None:
                val.next_val.prev_val = val.prev_val
            val.prev_val = None
            val.next_val = self._head
            self._head.prev_val = val
            self._head = val
        return val

    def get(self, key: int) -> int:
        val = self._get(key)
        if val is None:
            return -1
        return val.val

    def put(self, key: int, value: int) -> None:
        val = self._get(key)
        if val is not None:
            val.val = value
            return
        val = _Value(key, value)
        if self._head is not None:
            val.next_val = self._head
            self._head.prev_val = val
        self._head = val
        if self._last is None:
            self._last = val
        self._cache[key] = val
        if self._curr_cnt == self._cap:
            last = self._last
            if last and last.prev_val is not None:
                last.prev_val.next_val = None
                self._last = last.prev_val
            del self._cache[last.key]
        else:
            self._curr_cnt += 1


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
