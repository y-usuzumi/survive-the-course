from threading import Thread, Semaphore, Condition


class FooBar:
    def __init__(self, n):
        self.n = n
        self._sem = Semaphore(0)
        self._cond = Condition()

    def foo(self, printFoo: 'Callable[[], None]') -> None:
        self._sem.acquire()
        self._cond.acquire()
        for i in range(self.n):
            # printFoo() outputs "foo". Do not change or remove this line.
            printFoo()
            self._cond.notify()
            self._cond.wait()
        self._cond.notify()
        self._cond.release()

    def bar(self, printBar: 'Callable[[], None]') -> None:
        self._cond.acquire()
        self._sem.release()
        self._cond.wait()
        for i in range(self.n):
            # printBar() outputs "bar". Do not change or remove this line.
            printBar()
            self._cond.notify()
            self._cond.wait()
        self._cond.release()


if __name__ == '__main__':
    foobar = FooBar(n=1)
    def printFoo():
        print("foo", end="")
    def printBar(i):
        print("bar", end="")
    t1 = Thread(target=foobar.foo, args=(printFoo,))
    t2 = Thread(target=foobar.bar, args=(printBar,))
    t1.start()
    import time
    time.sleep(5)
    t2.start()
    t1.join()
    t2.join()
