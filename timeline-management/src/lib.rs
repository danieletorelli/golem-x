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
    authod_id: UserId,
    timestamp: i64,
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
                match action {
                    TimelineAction::Insert => {
                        entries.push(Entry {
                            id: tweet_id,
                            authod_id: user_id,
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

    //  get-timeline: func(user-id: string) -> result<list<string>>;
    fn get_timeline(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting timeline for user with id: {}", user_id);
        let user_id = UserId::from(user_id);
        STATE.with_borrow(|state| match state.entries.get(&user_id) {
            Some(entries) => Ok(entries.iter().map(|entry| entry.id.0.clone()).collect()),
            None => Err(()),
        })
    }
}

bindings::export!(Component with_types_in bindings);
