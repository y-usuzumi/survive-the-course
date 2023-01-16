// https://leetcode.com/problems/design-twitter/

use std::collections::{HashMap, HashSet, VecDeque};

type UserId = i32;
type Tweet = i32;
type Time = usize;

#[derive(Default)]
struct Twitter {
    user_tweets: HashMap<UserId, VecDeque<Time>>,
    following: HashMap<UserId, HashSet<UserId>>,
    max_tweets_per_user: usize,
    time_tweet_id: HashMap<usize, Tweet>,
    time: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            max_tweets_per_user: 10,
            ..Default::default()
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweets = self.user_tweets.entry(user_id).or_default();
        if tweets.len() == self.max_tweets_per_user {
            tweets.pop_back();
        }
        tweets.push_front(self.time);
        self.time_tweet_id.insert(self.time, tweet_id);
        self.time += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut all_users = self.following.entry(user_id).or_default().clone();
        all_users.insert(user_id);
        let user_tweets = all_users
            .iter()
            .map(|user| {
                self.user_tweets
                    .entry(*user)
                    .or_default()
                    .iter()
                    .take(self.max_tweets_per_user)
                    .map(|v| *v)
                    .collect::<Vec<Time>>()
            })
            .collect();
        return Self::merge_tweets(user_tweets, self.max_tweets_per_user)
            .into_iter()
            .map(|v| *self.time_tweet_id.get(&v).unwrap())
            .collect();
    }

    fn merge_tweets(mut user_tweets: Vec<Vec<Time>>, max_tweets: usize) -> Vec<Time> {
        match user_tweets.len() {
            0 => vec![],
            1 => {
                return std::mem::take(&mut user_tweets[0])
                    .into_iter()
                    .take(max_tweets)
                    .collect()
            }
            _ => {
                let left = user_tweets[0..user_tweets.len() / 2].to_vec();
                let right = user_tweets[user_tweets.len() / 2..].to_vec();
                let left_merged = Self::merge_tweets(left, max_tweets);
                let right_merged = Self::merge_tweets(right, max_tweets);
                let mut result = vec![];
                let mut left_idx = 0;
                let mut right_idx = 0;
                while left_idx < left_merged.len()
                    && right_idx < right_merged.len()
                    && result.len() < max_tweets
                {
                    if left_merged[left_idx] > right_merged[right_idx] {
                        result.push(left_merged[left_idx]);
                        left_idx += 1;
                    } else {
                        result.push(right_merged[right_idx]);
                        right_idx += 1;
                    }
                }
                while left_idx < left_merged.len() && result.len() < max_tweets {
                    result.push(left_merged[left_idx]);
                    left_idx += 1;
                }
                while right_idx < right_merged.len() && result.len() < max_tweets {
                    result.push(right_merged[right_idx]);
                    right_idx += 1;
                }

                return result;
            }
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.following
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

#[cfg(test)]
mod tests {
    use super::*;
}
