from threading import Thread, Semaphore, Condition, Lock

class ZeroEvenOdd:
    def __init__(self, n):
        self.n = n
        self._curr = 1
        self._even_sem = Semaphore(0)
        self._even_cond = Condition(lock=Lock())
        self._odd_sem = Semaphore(0)
        self._odd_cond = Condition(lock=Lock())

	# printNumber(x) outputs "x", where x is an integer.
    def zero(self, printNumber: 'Callable[[int], None]') -> None:
        self._odd_sem.acquire()
        self._even_sem.acquire()
        self._odd_cond.acquire()
        self._even_cond.acquire()
        while self._curr <= self.n:
            if self._curr % 2 == 1:
                printNumber(0)
                self._odd_cond.notify()
                self._odd_cond.wait()
                self._curr += 1
            else:
                printNumber(0)
                self._even_cond.notify()
                self._even_cond.wait()
                self._curr += 1
        self._odd_cond.notify()
        self._odd_cond.release()
        self._even_cond.notify()
        self._even_cond.release()

    def even(self, printNumber: 'Callable[[int], None]') -> None:
        self._even_cond.acquire()
        self._even_sem.release()
        self._even_cond.wait()
        while self._curr <= self.n:
            printNumber(self._curr)
            self._even_cond.notify()
            self._even_cond.wait()

    def odd(self, printNumber: 'Callable[[int], None]') -> None:
        self._odd_cond.acquire()
        self._odd_sem.release()
        self._odd_cond.wait()
        while self._curr <= self.n:
            printNumber(self._curr)
            self._odd_cond.notify()
            self._odd_cond.wait()

if __name__ == '__main__':
    zero_even_odd = ZeroEvenOdd(2)

    def printNumber(n):
        print(n)

    tzero = Thread(target=zero_even_odd.zero, args=(printNumber,))
    teven = Thread(target=zero_even_odd.even, args=(printNumber,))
    todd = Thread(target=zero_even_odd.odd, args=(printNumber,))

    tzero.start()
    teven.start()
    todd.start()
    tzero.join()
    teven.join()
    todd.join()
