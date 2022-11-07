package survive.the.course.java.leetcode._1195_Fizz_Buzz_Multithreaded;

import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;
import java.util.function.IntConsumer;

public class FizzBuzz {
    private int n;
    private volatile int curr;
    final Lock lock = new ReentrantLock();
    final Condition condFizz = lock.newCondition();
    final Condition condBuzz = lock.newCondition();
    final Condition condFizzBuzz = lock.newCondition();
    final Condition condNumber = lock.newCondition();

    public FizzBuzz(int n) {
        this.n = n;
        this.curr = 1;
    }

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        lock.lock();
        try {
            while (curr <= n) {
                condFizz.await();
                if (curr <= n) {
                    printFizz.run();
                    condNumber.signal();
                }
            }
        } finally {
            lock.unlock();
        }
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        lock.lock();
        try {
            while (curr <= n) {
                condBuzz.await();
                if (curr <= n) {
                    printBuzz.run();
                    condNumber.signal();
                }
            }
        } finally {
            lock.unlock();
        }
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        lock.lock();
        try {
            while (curr <= n) {
                condFizzBuzz.await();
                if (curr <= n) {
                    printFizzBuzz.run();
                    condNumber.signal();
                }
            }
        } finally {
            lock.unlock();
        }
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        lock.lock();
        try {
            while (curr <= n) {
                if (curr % 3 == 0) {
                    if (curr % 5 == 0) {
                        condFizzBuzz.signal();
                        condNumber.await();
                    } else {
                        condFizz.signal();
                        condNumber.await();
                    }
                } else if (curr % 5 == 0) {
                    condBuzz.signal();
                    condNumber.await();
                } else {
                    printNumber.accept(curr);
                }
                curr++;
            }
            condFizz.signal();
            condBuzz.signal();
            condFizzBuzz.signal();
        } finally {
            lock.unlock();
        }
    }
}
