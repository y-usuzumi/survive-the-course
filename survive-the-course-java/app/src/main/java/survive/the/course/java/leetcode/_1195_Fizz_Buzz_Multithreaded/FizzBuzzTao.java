package survive.the.course.java.leetcode._1195_Fizz_Buzz_Multithreaded;

import java.util.function.IntConsumer;

import com.google.common.base.Supplier;

public class FizzBuzzTao {
    private int n;
    private int counter;
    private boolean isStarted;

    public FizzBuzzTao(int n) {
        this.n = n;
        this.isStarted = true;
        this.counter = 1;
    }

    private void outputString(IntConsumer func, Supplier<Boolean> condition) throws InterruptedException {
        synchronized (this) {
            while (this.isStarted) {
                if (condition.get()) {
                    func.accept(this.counter);
                    if (this.counter == this.n)
                        this.isStarted = false;
                    else
                        this.counter++;
                    this.notifyAll();
                } else {
                    this.wait();
                }
            }
        }

    }

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        this.outputString(num -> printFizz.run(), () -> this.counter % 3 == 0 && this.counter % 5 != 0);
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        this.outputString(num -> printBuzz.run(), () -> this.counter % 5 == 0 && this.counter % 3 != 0);
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        this.outputString(num -> printFizzBuzz.run(), () -> this.counter % 5 == 0 && this.counter % 3 == 0);
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        this.outputString(printNumber, () -> this.counter % 5 != 0 && this.counter % 3 != 0);
    }
}
