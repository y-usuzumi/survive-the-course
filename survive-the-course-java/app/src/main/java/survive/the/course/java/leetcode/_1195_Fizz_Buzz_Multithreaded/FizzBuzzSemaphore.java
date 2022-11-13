package survive.the.course.java.leetcode._1195_Fizz_Buzz_Multithreaded;

import java.util.concurrent.Semaphore;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.IntConsumer;

public class FizzBuzzSemaphore {
    private int n;
    private AtomicInteger i = new AtomicInteger(1);
    private Semaphore sem = new Semaphore(1, true);

    public FizzBuzzSemaphore(int n) {
        this.n = n;
    }

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        while (true) {
            try {
                sem.acquire();
                if (i.get() > n) {
                    break;
                }
                if (i.get() % 3 == 0 && i.get() % 5 != 0) {
                    printFizz.run();
                    i.incrementAndGet();
                }
            } finally {
                sem.release();
            }
        }
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        while (true) {
            try {
                sem.acquire();
                if (i.get() > n) {
                    break;
                }
                if (i.get() % 3 != 0 && i.get() % 5 == 0) {
                    printBuzz.run();
                    i.incrementAndGet();
                }
            } finally {
                sem.release();
            }
        }
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        while (true) {
            try {
                sem.acquire();
                if (i.get() > n) {
                    break;
                }
                if (i.get() % 3 == 0 && i.get() % 5 == 0) {
                    printFizzBuzz.run();
                    i.incrementAndGet();
                }
            } finally {
                sem.release();
            }
        }
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        while (true) {
            try {
                sem.acquire();
                if (i.get() > n) {
                    break;
                }
                if (i.get() % 3 != 0 && i.get() % 5 != 0) {
                    printNumber.accept(i.get());
                    i.incrementAndGet();
                }
            } finally {
                sem.release();
            }
        }
    }
}
