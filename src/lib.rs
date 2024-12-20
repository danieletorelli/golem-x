#[allow(warnings)]
mod bindings;

use bindings::exports::component::golem_x_interface::timeline_api;
use bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet;
use bindings::exports::component::golem_x_interface::tweet_api;
use bindings::exports::component::golem_x_interface::tweet_api::PostedTweet;
use bindings::exports::component::golem_x_interface::user_api;
use bindings::exports::component::golem_x_interface::user_api::Username;
use std::cell::RefCell;
use std::collections::HashSet;

struct State {
    picture: Vec<u8>,
    followers: HashSet<Username>,
    followings: HashSet<Username>,
    tweets: Vec<PostedTweet>,
}

impl State {
    fn new() -> Self {
        Self {
            picture: Vec::new(),
            followers: HashSet::new(),
            followings: HashSet::new(),
            tweets: Vec::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
    static USERNAME: Username = std::env::var("GOLEM_WORKER_NAME")
        .expect("GOLEM_WORKER_NAME not set")
        .strip_prefix("user-")
        .expect("Invalid GOLEM_WORKER_NAME")
        .to_string();
}

fn get_worker_urn(username: &Username) -> bindings::golem::rpc::types::Uri {
    let component_id = std::env::var("GOLEM_COMPONENT_ID").expect("GOLEM_COMPONENT_ID not set");
    bindings::golem::rpc::types::Uri {
        value: format!("urn:worker:{component_id}/user-{}", username.to_lowercase()),
    }
}

fn check_target_username(user: &Username) -> bool {
    if *user == get_username() {
        eprintln!("Cannot perform actions on yourself");
        false
    } else {
        true
    }
}

fn get_username() -> Username {
    USERNAME.with(|u| u.clone())
}

struct Component;

impl user_api::Guest for Component {
    fn follow(user: Username) -> bool {
        use bindings::component::golem_x_stub::stub_golem_x::UserApi;

        if check_target_username(&user) {
            let username = get_username();
            let api = UserApi::new(&get_worker_urn(&user));
            if api.blocking_followed_by(&username) {
                println!("User '{}' is now following user '{}'", username, user);
                STATE.with_borrow_mut(|s| s.followings.insert(user))
            } else {
                false
            }
        } else {
            false
        }
    }

    fn followed_by(user: Username) -> bool {
        if check_target_username(&user) {
            let username = get_username();
            println!("User '{}' is now followed by user '{}'", username, user);
            STATE.with_borrow_mut(|s| s.followers.insert(user))
        } else {
            false
        }
    }

    fn get_followers() -> Vec<Username> {
        println!("Getting followers");
        STATE.with_borrow(|s| s.followers.iter().cloned().collect())
    }

    fn get_followings() -> Vec<Username> {
        println!("Getting followings");
        STATE.with_borrow(|s| s.followings.iter().cloned().collect())
    }

    fn get_picture() -> Vec<u8> {
        println!("Getting picture");
        STATE.with_borrow(|s| s.picture.clone())
    }

    fn unfollow(user: Username) -> bool {
        use bindings::component::golem_x_stub::stub_golem_x::UserApi;

        if check_target_username(&user) {
            let username = get_username();
            let api = UserApi::new(&get_worker_urn(&user));
            if api.blocking_unfollowed_by(&user) {
                println!("User '{}' is no longer following user '{}'", username, user);
                STATE.with_borrow_mut(|s| s.followings.remove(&user))
            } else {
                false
            }
        } else {
            false
        }
    }

    fn unfollowed_by(user: Username) -> bool {
        if check_target_username(&user) {
            let username = get_username();
            println!(
                "User '{}' is no longer followed by user '{}'",
                username, user
            );
            STATE.with_borrow_mut(|s| s.followers.remove(&user))
        } else {
            false
        }
    }

    fn update_picture(picture_data: Vec<u8>) -> bool {
        println!("Updating picture");
        STATE.with_borrow_mut(|s| {
            s.picture = picture_data;
        });
        true
    }
}

impl tweet_api::Guest for Component {
    fn get_tweets(limit: u8) -> Vec<PostedTweet> {
        println!("Getting tweets");
        STATE
            .with_borrow(|s| s.tweets.clone())
            .into_iter()
            .rev()
            .take(limit as usize)
            .collect()
    }

    fn post_tweet(content: String) -> PostedTweet {
        println!("Posting tweet: {}", content);
        let tweet = PostedTweet {
            content,
            timestamp: chrono::Utc::now().timestamp(),
        };
        STATE.with_borrow_mut(|s| {
            s.tweets.push(tweet.clone());
        });
        tweet
    }
}

impl timeline_api::Guest for Component {
    fn get_timeline() -> Vec<TimelineTweet> {
        use bindings::component::golem_x_stub::stub_golem_x::TweetApi;

        println!("Getting timeline");
        let mut timeline: Vec<TimelineTweet> = STATE
            .with_borrow(|s| s.followings.clone())
            .iter()
            .flat_map(|username| {
                let api = TweetApi::new(&get_worker_urn(username));
                api.blocking_get_tweets(50)
                    .into_iter()
                    .map(|tweet| TimelineTweet {
                        author: username.to_string(),
                        content: tweet.content,
                        timestamp: tweet.timestamp,
                    })
            })
            .collect();
        timeline.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        timeline
    }
}

bindings::export!(Component with_types_in bindings);
