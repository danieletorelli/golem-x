#[allow(warnings)]
mod bindings;

use bindings::exports::component::tweet_management::tweet_api::*;
use chrono::Utc;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TweetId(u64);

#[derive(Debug, Clone)]
struct Tweet {
    id: TweetId,
    author_id: UserId,
    content: String,
    timestamp: i64,
}

impl Tweet {
    fn new(id: TweetId, content: String, author_id: UserId) -> Self {
        Tweet {
            id,
            author_id,
            content,
            timestamp: Utc::now().timestamp(),
        }
    }

    fn to_posted_tweet(&self) -> PostedTweet {
        PostedTweet {
            tweet_id: self.id.0,
            author_id: self.author_id.0,
            content: self.content.clone(),
            timestamp: self.timestamp,
        }
    }
}

struct State {
    tweets_count: u64,
    tweets: BTreeMap<UserId, Vec<Tweet>>,
}

impl State {
    fn new() -> Self {
        State {
            tweets_count: 0,
            tweets: BTreeMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

struct Component;

impl Guest for Component {
    fn post_tweet(user_id: u64, content: String) -> Result<PostedTweet, ()> {
        println!("User with id: {} posted tweet: {}", user_id, content);
        let user_id = UserId(user_id);
        STATE.with_borrow_mut(|state| {
            let tweet_id = TweetId(state.tweets_count);
            let tweet = Tweet::new(tweet_id.clone(), content, user_id.clone());
            state.tweets.entry(user_id).or_default().push(tweet.clone());
            state.tweets_count += 1;
            Ok(tweet.to_posted_tweet())
        })
    }

    fn get_user_tweets(user_id: u64) -> Result<Vec<PostedTweet>, ()> {
        println!("Getting tweets for user with id: {}", user_id);
        let user_id = UserId(user_id);
        STATE.with_borrow(|state| match state.tweets.get(&user_id) {
            Some(tweets) => Ok(tweets
                .iter()
                .map(|tweet| tweet.to_posted_tweet().clone())
                .collect()),
            None => Ok(vec![]),
        })
    }

    fn get_specific_tweets(user_id: u64, tweet_ids: Vec<u64>) -> Result<Vec<PostedTweet>, ()> {
        println!("Getting specific tweets for user with id: {}", user_id);
        Self::get_user_tweets(user_id).map(|tweets| {
            tweets
                .into_iter()
                .filter(|tweet| tweet_ids.contains(&tweet.tweet_id))
                .collect()
        })
    }
}

bindings::export!(Component with_types_in bindings);
