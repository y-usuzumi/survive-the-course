package survive.the.course.java.neetcode.heap_or_priority_queue.design_twitter;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

class TimeTweetIdPair {
    public int time;
    public int tweetId;

    public TimeTweetIdPair(final int time, final int tweetId) {
        this.time = time;
        this.tweetId = tweetId;
    }
}

public class Twitter {
    private Map<Integer, LinkedList<TimeTweetIdPair>> tweets = new HashMap<>();
    private Map<Integer, Set<Integer>> followees = new HashMap<>();
    private static int time = 0;

    public Twitter() {

    }

    public void postTweet(int userId, int tweetId) {
        LinkedList<TimeTweetIdPair> userTweets = getOrInsertDefault(tweets, userId, new LinkedList<>());
        userTweets.addFirst(new TimeTweetIdPair(getTime(), tweetId));
    }

    public List<Integer> getNewsFeed(int userId) {
        List<LinkedList<TimeTweetIdPair>> allUserTweets = new ArrayList<>();
        allUserTweets.add(tweets.getOrDefault(userId, new LinkedList<>()));
        for (int followeeId : followees.getOrDefault(userId, new HashSet<>())) {
            allUserTweets.add(tweets.getOrDefault(followeeId, new LinkedList<>()));
        }

        return mergeTake(allUserTweets, 10)
                .stream()
                .map(pair -> pair.tweetId).collect(Collectors.toList());
    }

    public void follow(int followerId, int followeeId) {
        Set<Integer> userFollowees = getOrInsertDefault(followees, followerId, new HashSet<>());
        userFollowees.add(followeeId);
    }

    public void unfollow(int followerId, int followeeId) {
        Set<Integer> userFollowees = getOrInsertDefault(followees, followerId, new HashSet<>());
        userFollowees.remove(followeeId);
    }

    private static int getTime() {
        return time++;
    }

    private <K, V> V getOrInsertDefault(Map<K, V> map, K key, V defaultValue) {
        if (!map.containsKey(key)) {
            map.put(key, defaultValue);
            return defaultValue;
        } else {
            return map.get(key);
        }
    }

    private LinkedList<TimeTweetIdPair> mergeTake(List<LinkedList<TimeTweetIdPair>> lists, int num) {
        switch (lists.size()) {
            case 0:
                return new LinkedList<>();
            case 1:
                return lists.get(0).stream().limit(num).collect(Collectors.toCollection(LinkedList::new));
            case 2:
                return mergeTakeTwo(lists.get(0), lists.get(1), num);
            default:
                return mergeTakeTwo(
                        mergeTake(lists.subList(0, lists.size() / 2), num),
                        mergeTake(lists.subList(lists.size() / 2, lists.size()), num),
                        num);
        }
    }

    private LinkedList<TimeTweetIdPair> mergeTakeTwo(LinkedList<TimeTweetIdPair> left,
            LinkedList<TimeTweetIdPair> right, int num) {
        var result = new LinkedList<TimeTweetIdPair>();
        var leftIterator = left.listIterator();
        var rightIterator = right.listIterator();
        while (leftIterator.hasNext() && rightIterator.hasNext()) {
            var currLeft = leftIterator.next();
            var currRight = rightIterator.next();
            if (currLeft.time > currRight.time) {
                result.add(currLeft);
                rightIterator.previous();
            } else {
                result.add(currRight);
                leftIterator.previous();
            }
            if (result.size() == num) {
                return result;
            }
        }
        while (leftIterator.hasNext()) {
            result.add(leftIterator.next());
            if (result.size() == num) {
                return result;
            }
        }

        while (rightIterator.hasNext()) {
            result.add(rightIterator.next());
            if (result.size() == num) {
                return result;
            }
        }
        return result;
    }
}
