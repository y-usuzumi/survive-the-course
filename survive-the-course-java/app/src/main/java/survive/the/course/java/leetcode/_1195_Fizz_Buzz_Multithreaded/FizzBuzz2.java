// https://leetcode.cn/problems/fizz-buzz-multithreaded/
package survive.the.course.java.leetcode._1195_Fizz_Buzz_Multithreaded;

import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.IntConsumer;

public class FizzBuzz2 {
    private int n;
    private AtomicBoolean canRunFizz = new AtomicBoolean();
    private AtomicBoolean canRunBuzz = new AtomicBoolean();
    private AtomicBoolean canRunFizzBuzz = new AtomicBoolean();
    private AtomicBoolean finished = new AtomicBoolean();

    public FizzBuzz2(int n) {
        this.n = n;
    }

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        synchronized (canRunFizz) {
            while (!canRunFizz.get()) {
                canRunFizz.wait();
            }
        }

        synchronized (canRunFizz) {
            while (true) {
                canRunFizz.notify();
                canRunFizz.wait();
                if (finished.get()) {
                    break;
                }
                printFizz.run();
            }
        }
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        synchronized (canRunBuzz) {
            while (!canRunBuzz.get()) {
                canRunBuzz.wait();
            }
        }

        synchronized (canRunBuzz) {
            while (true) {
                canRunBuzz.notify();
                canRunBuzz.wait();
                if (finished.get()) {
                    break;
                }
                printBuzz.run();
            }
        }
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        synchronized (canRunFizzBuzz) {
            while (!canRunFizzBuzz.get()) {
                canRunFizzBuzz.wait();
            }
        }

        synchronized (canRunFizzBuzz) {
            while (true) {
                canRunFizzBuzz.notify();
                canRunFizzBuzz.wait();
                if (finished.get()) {
                    break;
                }
                printFizzBuzz.run();
            }
        }
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        synchronized (canRunFizz) {
            canRunFizz.set(true);
            canRunFizz.notify();
            canRunFizz.wait();
        }
        synchronized (canRunBuzz) {
            canRunBuzz.set(true);
            canRunBuzz.notify();
            canRunBuzz.wait();
        }
        synchronized (canRunFizzBuzz) {
            canRunFizzBuzz.set(true);
            canRunFizzBuzz.notify();
            canRunFizzBuzz.wait();
        }

        synchronized (canRunFizz) {
            synchronized (canRunBuzz) {
                synchronized (canRunFizzBuzz) {
                    for (int i = 0; i < n; i++) {
                        if (i % 3 == 0) {
                            if (i % 5 == 0) {
                                canRunFizzBuzz.notify();
                                canRunFizzBuzz.wait();
                            } else {
                                canRunFizz.notify();
                                canRunFizz.wait();
                            }
                        } else if (i % 5 == 0) {
                            canRunBuzz.notify();
                            canRunBuzz.wait();
                        } else {
                            printNumber.accept(i);
                        }
                    }
                    finished.set(true);
                    canRunFizz.notify();
                    canRunBuzz.notify();
                    canRunFizzBuzz.notify();
                }
            }
        }
    }
}