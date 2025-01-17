#[allow(warnings)]
mod bindings;
mod state;
mod timeline_cache;
mod util;

use bindings::exports::component::golem_x_exports::timeline_api;
use bindings::exports::component::golem_x_exports::tweet_api;
use bindings::exports::component::golem_x_exports::user_api;

fn get_worker_urn(username: &user_api::Username) -> bindings::golem::rpc::types::Uri {
    let component_id = std::env::var("GOLEM_COMPONENT_ID").expect("GOLEM_COMPONENT_ID not set");
    bindings::golem::rpc::types::Uri {
        value: format!("urn:worker:{component_id}/user-{}", username.to_lowercase()),
    }
}

fn check_target_username(user: &user_api::Username) -> bool {
    if *user == state::get_username() {
        eprintln!("Cannot perform actions on yourself");
        false
    } else {
        true
    }
}

struct Component;

impl user_api::Guest for Component {
    fn follow(user: user_api::Username) -> bool {
        use bindings::component::golem_x_client::golem_x_client::UserApi;

        if !check_target_username(&user) {
            return false;
        }
        let username = state::get_username();
        let api = UserApi::new(&get_worker_urn(&user));
        if !api.blocking_followed_by(&username) {
            return false;
        }

        println!("User '{}' is now following user '{}'", username, user);
        state::update(|s| {
            if s.followings.insert(user) {
                timeline_cache::invalidate();
                true
            } else {
                false
            }
        })
    }

    fn followed_by(user: user_api::Username) -> bool {
        if check_target_username(&user) {
            let username = state::get_username();
            println!("User '{}' is now followed by user '{}'", username, user);
            state::update(|s| s.followers.insert(user))
        } else {
            false
        }
    }

    fn get_followers() -> Vec<user_api::Username> {
        println!("Getting followers");
        state::update(|s| s.followers.iter().cloned().collect())
    }

    fn get_followings() -> Vec<user_api::Username> {
        println!("Getting followings");
        state::update(|s| s.followings.iter().cloned().collect())
    }

    fn get_picture() -> Vec<u8> {
        println!("Getting picture");
        state::get(|s| s.picture.clone())
    }

    fn unfollow(user: user_api::Username) -> bool {
        use bindings::component::golem_x_client::golem_x_client::UserApi;

        if !check_target_username(&user) {
            return false;
        }
        let username = state::get_username();
        let api = UserApi::new(&get_worker_urn(&user));
        if !api.blocking_unfollowed_by(&username) {
            return false;
        }

        println!("User '{}' is no longer following user '{}'", username, user);
        state::update(|s| {
            if s.followings.remove(&user) {
                timeline_cache::invalidate();
                true
            } else {
                false
            }
        })
    }

    fn unfollowed_by(user: user_api::Username) -> bool {
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
    fn get_tweets(limit: u8) -> Vec<tweet_api::PostedTweet> {
        println!("Getting tweets");
        state::get(|s| s.tweets.clone())
            .into_iter()
            .rev()
            .take(limit as usize)
            .collect()
    }

    fn post_tweet(content: String) -> tweet_api::PostedTweet {
        use bindings::component::golem_x_client::golem_x_client::TimelineApi;

        println!("Posting tweet: {}", content);
        let tweet = tweet_api::PostedTweet::from(content);
        state::update(|s| s.tweets.push(tweet.clone()));
        for follower in state::get(|s| s.followers.clone()) {
            let api = TimelineApi::new(&get_worker_urn(&follower));
            api.blocking_invalidate_timeline_cache();
        }
        tweet
    }
}

impl timeline_api::Guest for Component {
    fn get_timeline() -> Vec<timeline_api::TimelineTweet> {
        use bindings::component::golem_x_client::golem_x_client::TweetApi;

        println!("Getting timeline");
        timeline_cache::get().unwrap_or_else(|| {
            let mut timeline: Vec<timeline_api::TimelineTweet> =
                state::get(|s| s.followings.clone())
                    .iter()
                    .flat_map(|username| {
                        let api = TweetApi::new(&get_worker_urn(username));
                        api.blocking_get_tweets(50)
                            .into_iter()
                            .map(move |tweet| timeline_api::TimelineTweet::from((tweet, username)))
                    })
                    .collect();
            timeline.sort_by_key(|tweet| tweet.timestamp);
            timeline_cache::update(timeline.clone());
            timeline
        })
    }

    fn invalidate_timeline_cache() {
        println!("Invalidating timeline cache");
        timeline_cache::invalidate();
    }
}

bindings::export!(Component with_types_in bindings);
