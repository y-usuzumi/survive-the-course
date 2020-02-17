from threading import Semaphore

class Foo:
    def __init__(self):
        self._sem_2 = Semaphore(0)
        self._sem_3 = Semaphore(0)


    def first(self, printFirst: 'Callable[[], None]') -> None:
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self._sem_2.release()


    def second(self, printSecond: 'Callable[[], None]') -> None:
        self._sem_2.acquire()
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self._sem_3.release()


    def third(self, printThird: 'Callable[[], None]') -> None:
        self._sem_3.acquire()
        # printThird() outputs "third". Do not change or remove this line.
        printThird()
