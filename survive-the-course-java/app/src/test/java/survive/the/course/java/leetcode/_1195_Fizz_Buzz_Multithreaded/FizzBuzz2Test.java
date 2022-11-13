package survive.the.course.java.leetcode._1195_Fizz_Buzz_Multithreaded;

import org.junit.jupiter.api.Test;

public class FizzBuzz2Test {
    @Test
    void test1() throws InterruptedException {
        FizzBuzz2 fb = new FizzBuzz2(20);
        Thread t1 = new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    fb.fizz(new Runnable() {
                        @Override
                        public void run() {
                            System.out.println("Fizz");
                        }
                    });
                } catch (InterruptedException e) {
                    // TODO Auto-generated catch block
                    e.printStackTrace();
                }
            }
        });
        Thread t2 = new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    fb.buzz(new Runnable() {
                        @Override
                        public void run() {
                            System.out.println("Buzz");
                        }
                    });
                } catch (InterruptedException e) {
                    // TODO Auto-generated catch block
                    e.printStackTrace();
                }
            }
        });
        Thread t3 = new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    fb.fizzbuzz(new Runnable() {
                        @Override
                        public void run() {
                            System.out.println("FizzBuzz");
                        }
                    });
                } catch (InterruptedException e) {
                    // TODO Auto-generated catch block
                    e.printStackTrace();
                }
            }
        });
        Thread t4 = new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    fb.number(num -> {
                        System.out.println(num);
                    });
                } catch (InterruptedException e) {
                    // TODO Auto-generated catch block
                    e.printStackTrace();
                }
            }
        });
        t4.start();
        t1.start();
        t2.start();
        t3.start();
        t1.join();
        t2.join();
        t3.join();
        t4.join();
    }
}
