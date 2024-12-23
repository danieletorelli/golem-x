#[allow(warnings)]
mod bindings;
mod state;
mod timeline_cache;
mod util;

use bindings::exports::component::golem_x_interface::timeline_api;
use bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet;
use bindings::exports::component::golem_x_interface::tweet_api;
use bindings::exports::component::golem_x_interface::tweet_api::PostedTweet;
use bindings::exports::component::golem_x_interface::user_api;
use bindings::exports::component::golem_x_interface::user_api::Username;

fn get_worker_urn(username: &Username) -> bindings::golem::rpc::types::Uri {
    let component_id = std::env::var("GOLEM_COMPONENT_ID").expect("GOLEM_COMPONENT_ID not set");
    bindings::golem::rpc::types::Uri {
        value: format!("urn:worker:{component_id}/user-{}", username.to_lowercase()),
    }
}

fn check_target_username(user: &Username) -> bool {
    if *user == state::get_username() {
        eprintln!("Cannot perform actions on yourself");
        false
    } else {
        true
    }
}

struct Component;

impl user_api::Guest for Component {
    fn follow(user: Username) -> bool {
        use bindings::component::golem_x_stub::stub_golem_x::UserApi;

        if check_target_username(&user) {
            let username = state::get_username();
            let api = UserApi::new(&get_worker_urn(&user));
            if api.blocking_followed_by(&username) {
                println!("User '{}' is now following user '{}'", username, user);
                state::update(|s| {
                    if s.followings.insert(user) {
                        timeline_cache::invalidate();
                        true
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn followed_by(user: Username) -> bool {
        if check_target_username(&user) {
            let username = state::get_username();
            println!("User '{}' is now followed by user '{}'", username, user);
            state::update(|s| s.followers.insert(user))
        } else {
            false
        }
    }

    fn get_followers() -> Vec<Username> {
        println!("Getting followers");
        state::update(|s| s.followers.iter().cloned().collect())
    }

    fn get_followings() -> Vec<Username> {
        println!("Getting followings");
        state::update(|s| s.followings.iter().cloned().collect())
    }

    fn get_picture() -> Vec<u8> {
        println!("Getting picture");
        state::get(|s| s.picture.clone())
    }

    fn unfollow(user: Username) -> bool {
        use bindings::component::golem_x_stub::stub_golem_x::UserApi;

        if check_target_username(&user) {
            let username = state::get_username();
            let api = UserApi::new(&get_worker_urn(&user));
            if api.blocking_unfollowed_by(&user) {
                println!("User '{}' is no longer following user '{}'", username, user);
                state::update(|s| {
                    if s.followings.remove(&user) {
                        timeline_cache::invalidate();
                        true
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn unfollowed_by(user: Username) -> bool {
        if check_target_username(&user) {
            let username = state::get_username();
            println!(
                "User '{}' is no longer followed by user '{}'",
                username, user
            );
            state::update(|s| s.followers.remove(&user))
        } else {
            false
        }
    }

    fn update_picture(picture_data: Vec<u8>) -> bool {
        println!("Updating picture");
        state::update(|s| s.picture = picture_data);
        true
    }
}

impl tweet_api::Guest for Component {
    fn get_tweets(limit: u8) -> Vec<PostedTweet> {
        println!("Getting tweets");
        state::get(|s| s.tweets.clone())
            .into_iter()
            .rev()
            .take(limit as usize)
            .collect()
    }

    fn post_tweet(content: String) -> PostedTweet {
        println!("Posting tweet: {}", content);
        let tweet = PostedTweet::from(content);
        state::update(|s| s.tweets.push(tweet.clone()));
        timeline_cache::invalidate();
        tweet
    }
}

impl timeline_api::Guest for Component {
    fn get_timeline() -> Vec<TimelineTweet> {
        use bindings::component::golem_x_stub::stub_golem_x::TweetApi;

        println!("Getting timeline");
        timeline_cache::get().unwrap_or_else(|| {
            let mut timeline: Vec<TimelineTweet> = state::get(|s| s.followings.clone())
                .iter()
                .flat_map(|username| {
                    let api = TweetApi::new(&get_worker_urn(username));
                    api.blocking_get_tweets(50)
                        .into_iter()
                        .map(move |tweet| TimelineTweet::from((tweet, username)))
                })
                .collect();
            timeline.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            timeline_cache::update(timeline.clone());
            timeline
        })
    }
}

bindings::export!(Component with_types_in bindings);
