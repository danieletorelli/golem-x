#[allow(warnings)]
mod bindings;

use bindings::exports::component::router::router_api::*;
use blake2::Digest;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Worker {
    id: String,
}

thread_local! {
    static WORKERS: RefCell<BTreeMap<u64, Worker>> = const { RefCell::new(BTreeMap::new()) };
}

struct Component;

fn hash(input: &str) -> u64 {
    let mut hasher = blake2::Blake2b512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    // Convert the first 8 bytes of the hash into u64
    let bytes: [u8; 8] = result[..8].try_into().expect("slice with incorrect length");
    u64::from_le_bytes(bytes)
}

fn get_user_worker_urn(user_id: String) -> String {
    let worker_id = get_responsible_worker(user_id.clone());
    let component_id = std::env::var("USER_MANAGEMENT_COMPONENT_ID")
        .expect("USER_MANAGEMENT_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/user-manager-{worker_id}")
}
fn get_tweet_worker_urn() -> String {
    let component_id = std::env::var("TWEET_MANAGEMENT_COMPONENT_ID")
        .expect("TWEET_MANAGEMENT_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/tweet-manager")
}
fn get_timeline_worker_urn() -> String {
    let component_id =
        std::env::var("TIMELINE_COMPONENT_ID").expect("TIMELINE_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/timeline-manager")
}

fn add_worker(worker_id: String) -> bool {
    println!("Adding worker with id: {}", worker_id);
    WORKERS
        .with_borrow_mut(|workers| {
            workers.insert(hash(worker_id.as_str()), Worker { id: worker_id })
        })
        .is_some()
    // TODO: Rebalance workers data
}

#[allow(unused)]
fn remove_worker(worker_id: String) -> bool {
    println!("Removing worker with id: {}", worker_id);
    WORKERS
        .with_borrow_mut(|workers| workers.remove(&hash(worker_id.as_str())))
        .is_some()
    // TODO: Rebalance workers data
}

fn get_responsible_worker(key: String) -> String {
    println!("Getting responsible worker for key: {}", key);
    WORKERS.with_borrow(|workers| {
        let worker_hash = hash(key.as_str());
        match workers.get(&worker_hash) {
            Some(worker) => {
                println!("Responsible worker for key '{}' found: {}", key, worker.id);
                worker.id.clone()
            }
            None => {
                let worker_id = worker_hash.to_string();
                println!(
                    "Responsible worker for key '{}' not found. Will be created with id {}.",
                    key, worker_id
                );
                add_worker(worker_id.clone());
                worker_id
            }
        }
    })
}

fn post_tweet(user_id: String, content: String) -> Result<String, ()> {
    println!(
        "Posting tweet for user with id: {} with content: {}",
        user_id.clone(),
        content
    );

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(),
    });

    api.blocking_post_tweet(user_id.as_str(), content.as_str())
}

fn get_tweets(user_id: String) -> Result<Vec<String>, ()> {
    println!("Getting tweets for user with id: {}", user_id.clone());

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(),
    });

    api.blocking_get_user_tweets(user_id.as_str())
}

fn update_timeline(user_id: String, tweet_id: String) -> Result<bool, ()> {
    println!(
        "Updating timeline for user with id: {} with tweet with id: {}",
        user_id.clone(),
        tweet_id.clone()
    );

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    let api = TimelineApi::new(&Uri {
        value: get_timeline_worker_urn(),
    });

    api.blocking_update_timeline(user_id.as_str(), tweet_id.as_str())
}

fn get_timeline(user_id: String) -> Result<Vec<String>, ()> {
    println!("Getting timeline for user with id: {}", user_id.clone());

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    let api = TimelineApi::new(&Uri {
        value: get_timeline_worker_urn(),
    });

    api.blocking_get_timeline(user_id.as_str())
}

fn get_username(urn: String, user_id: String) -> Result<String, ()> {
    println!(
        "Calling {} to get username for user with id: {}",
        urn.clone(),
        user_id.clone()
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_username(user_id.as_str())
}

fn get_followers(urn: String, user_id: String) -> Result<Vec<String>, ()> {
    println!(
        "Calling {} to get username for user with id: {}",
        urn.clone(),
        user_id.clone()
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_followers(user_id.as_str())
}

fn get_following(urn: String, user_id: String) -> Result<Vec<String>, ()> {
    println!(
        "Calling {} to get username for user with id: {}",
        urn.clone(),
        user_id.clone()
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_following(user_id.as_str())
}

fn compose_user_profile(urn: String, user_id: String) -> Result<UserProfile, String> {
    println!(
        "Calling {} to compose user profile for user {}",
        urn, user_id
    );

    let username_result = get_username(urn.clone(), user_id.clone());
    let followers_result = get_followers(urn.clone(), user_id.clone());
    let following_result = get_following(urn, user_id.clone());

    match (username_result, followers_result, following_result) {
        (Ok(username), Ok(followers), Ok(following)) => Ok(UserProfile {
            user_id,
            username,
            followers,
            following,
        }),
        (Err(_), _, _) => Err("Failed to get username".to_string()),
        (_, Err(_), _) => Err("Failed to get followers".to_string()),
        (_, _, Err(_)) => Err("Failed to get following".to_string()),
    }
}

fn compose_tweets(urn: String, user_id: String) -> Result<TweetData, ()> {
    println!("Calling {} to compose tweets for user {}", urn, user_id);

    get_tweets(user_id.clone()).map(|tweets| TweetData { user_id, tweets })
}

fn compose_timeline(urn: String, user_id: String) -> Result<Vec<TimelineData>, ()> {
    println!("Calling {} to compose timeline for user {}", urn, user_id);

    get_timeline(user_id.clone()).map(|tweets| {
        tweets
            .into_iter()
            .map(|tweet_id| TimelineData {
                tweet_id,
                user_id: user_id.clone(),
            })
            .collect()
    })
}

impl Guest for Component {
    // notify-new-tweet: func(tweet-id: string, user-id: string, followers: list<string>) -> result<bool>;
    fn notify_new_tweet(
        tweet_id: String,
        user_id: String,
        _followers: Vec<String>,
    ) -> Result<bool, ()> {
        println!(
            "Notifying followers of user with id: {} about new tweet with id: {}",
            user_id, tweet_id
        );
        Ok(true)
    }

    // inform-follow-change: func(user-id: string, target-user-id: string, action: action) -> result<bool>;
    fn inform_follow_change(
        user_id: String,
        target_user_id: String,
        action: Action,
    ) -> Result<bool, ()> {
        println!(
            "Informing user with id: {} about follow change for user with id: {} with action: {:?}",
            user_id, target_user_id, action
        );
        Ok(true)
    }

    // route-query: func(query-type: query-type, key: string, data: string) -> result<string>;
    fn route_query(query_type: QueryType, key: String, data: String) -> RouterResponse {
        use bindings::exports::component::router::router_api::RouterResponse::*;
        println!(
            "Routing query of type: {:?} for key: {} with data: {}",
            query_type, key, data
        );
        match query_type {
            QueryType::UserProfile => {
                println!("Query type: UserProfile");
                let user_id = key;
                let worker_urn = get_user_worker_urn(user_id.clone());
                compose_user_profile(worker_urn, user_id)
                    .map(UserProfileResponse)
                    .unwrap_or_else(Failure)
            }
            QueryType::UserFollowers => {
                println!("Query type: UserFollowers");
                let user_id = key;
                let worker_urn = get_user_worker_urn(user_id.clone());
                get_followers(worker_urn, user_id)
                    .map(UserFollowersResponse)
                    .unwrap_or_else(|_| Failure("Failed to get followers".to_string()))
            }
            QueryType::UserFollowing => {
                println!("Query type: UserFollowing");
                let user_id = key;
                let worker_urn = get_user_worker_urn(user_id.clone());
                get_following(worker_urn, user_id)
                    .map(UserFollowingResponse)
                    .unwrap_or_else(|_| Failure("Failed to get following".to_string()))
            }
            QueryType::UserTweets => {
                println!("Query type: UserTweets");
                let user_id = key;
                compose_tweets(get_tweet_worker_urn(), user_id)
                    .map(UserTweetsResponse)
                    .unwrap_or_else(|_| Failure("Failed to get tweets".to_string()))
            }
            QueryType::TweetData => {
                println!("Query type: TweetData");
                Failure("Not implemented".to_string())
            }
            QueryType::UserTimeline => {
                println!("Query type: UserTimeline");
                let user_id = key;
                compose_timeline(get_timeline_worker_urn(), user_id)
                    .map(UserTimelineResponse)
                    .unwrap_or_else(|_| Failure("Failed to get timeline".to_string()))
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
