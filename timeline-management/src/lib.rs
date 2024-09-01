#[allow(warnings)]
mod bindings;

use bindings::exports::component::timeline_management::timeline_api::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(String);

impl UserId {
    fn from(s: String) -> Self {
        UserId(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TweetId(String);

impl TweetId {
    fn from(s: String) -> Self {
        TweetId(s)
    }
}

#[derive(Debug)]
struct Entry {
    id: TweetId,
    author_id: UserId,
    timestamp: i64,
}

impl Entry {
    fn to_timeline_tweet(&self) -> TimelineTweet {
        TimelineTweet {
            tweet_id: self.id.0.clone(),
            author_id: self.author_id.0.clone(),
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
    //  update-timeline: func(user-id: string, tweet-id: string, timestamp: s64, action: timeline-action) -> result<bool>;
    fn update_timeline(
        user_id: String,
        tweet_id: String,
        author_id: String,
        timestamp: i64,
        action: TimelineAction,
    ) -> Result<bool, ()> {
        println!(
            "Updating timeline for user with id: {} with tweet id: {}",
            user_id, tweet_id
        );
        let user_id = UserId::from(user_id);
        STATE.with_borrow_mut(|state| match state.entries.get_mut(&user_id) {
            Some(entries) => {
                let tweet_id = TweetId::from(tweet_id);
                let author_id = UserId::from(author_id);
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
            }
            None => Err(()),
        })
    }

    //  get-timeline: func(user-id: string) -> result<list<timeline-tweet>>;
    fn get_timeline(user_id: String) -> Result<Vec<TimelineTweet>, ()> {
        println!("Getting timeline for user with id: {}", user_id);
        let user_id = UserId::from(user_id);
        STATE.with_borrow(|state| match state.entries.get(&user_id) {
            Some(entries) => Ok(entries
                .iter()
                .map(|entry| entry.to_timeline_tweet())
                .collect()),
            None => Err(()),
        })
    }
}

bindings::export!(Component with_types_in bindings);
