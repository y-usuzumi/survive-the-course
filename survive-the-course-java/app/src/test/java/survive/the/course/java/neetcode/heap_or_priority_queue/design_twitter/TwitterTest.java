package survive.the.course.java.neetcode.heap_or_priority_queue.design_twitter;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;

import org.junit.jupiter.api.Test;

public class TwitterTest {
    @Test
    void test1() {
        var twitter = new Twitter();
        twitter.postTweet(1, 5);
        twitter.follow(1, 2);
        twitter.follow(2, 1);
        twitter.getNewsFeed(2);
        twitter.postTweet(2, 6);
        assertEquals(Arrays.asList(6, 5), twitter.getNewsFeed(1)); ;
        assertEquals(Arrays.asList(6, 5), twitter.getNewsFeed(2));
    }
}
