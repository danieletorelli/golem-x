#[allow(warnings)]
mod bindings;

use bindings::exports::component::timeline_management::timeline_api::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TweetId(u64);

#[derive(Debug)]
struct Entry {
    id: TweetId,
    author_id: UserId,
    timestamp: i64,
}

impl Entry {
    fn to_timeline_tweet(&self) -> TimelineTweet {
        TimelineTweet {
            tweet_id: self.id.0,
            author_id: self.author_id.0,
            timestamp: self.timestamp,
        }
    }
}

struct State {
    entries: BTreeMap<UserId, Vec<Entry>>,
}

impl State {
    fn new() -> Self {
        State {
            entries: BTreeMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

struct Component;

impl Guest for Component {
    fn update_timeline(
        user_id: u64,
        tweet_id: u64,
        author_id: u64,
        timestamp: i64,
        action: TimelineAction,
    ) -> Result<bool, ()> {
        println!(
            "Updating timeline for user with id: {} with tweet id: {}",
            user_id, tweet_id
        );
        let user_id = UserId(user_id);
        let tweet_id = TweetId(tweet_id);
        let author_id = UserId(author_id);
        STATE.with_borrow_mut(|state| {
            let entries = state.entries.entry(user_id).or_default();
            match action {
                TimelineAction::Insert => {
                    entries.push(Entry {
                        id: tweet_id,
                        author_id,
                        timestamp,
                    });
                    entries.sort_by_key(|entry| entry.timestamp);
                    Ok(true)
                }
                TimelineAction::Remove => {
                    entries.retain(|entry| entry.id != tweet_id);
                    Ok(true)
                }
            }
        })
    }

    fn get_timeline(user_id: u64) -> Result<Vec<TimelineTweet>, ()> {
        println!("Getting timeline for user with id: {}", user_id);
        let user_id = UserId(user_id);
        STATE.with_borrow(|state| match state.entries.get(&user_id) {
            Some(entries) => Ok(entries
                .iter()
                .map(|entry| entry.to_timeline_tweet())
                .collect()),
            None => Ok(vec![]),
        })
    }
}

bindings::export!(Component with_types_in bindings);
