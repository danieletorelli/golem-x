#[allow(warnings)]
mod bindings;

use bindings::exports::component::router::router_api::*;
use blake2::Digest;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
struct Worker {
    id: String,
}

struct State {
    workers_count: u64,
    workers: BTreeMap<u64, Worker>,
}

impl State {
    fn new() -> Self {
        State {
            workers_count: 0,
            workers: BTreeMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
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
    let worker_id = match get_responsible_worker(user_id.clone()) {
        Some(worker) => worker.id,
        None => {
            add_worker(user_id.clone())
                .unwrap_or_else(|| panic!("Failed to add worker for user with id: {}", user_id))
                .id
        }
    };
    let component_id =
        std::env::var("USER_MANAGER_COMPONENT_ID").expect("USER_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/user-manager-{worker_id}")
}
fn get_tweet_worker_urn() -> String {
    let component_id =
        std::env::var("TWEET_MANAGER_COMPONENT_ID").expect("TWEET_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/tweet-manager")
}
fn get_timeline_worker_urn() -> String {
    let component_id = std::env::var("TIMELINE_MANAGER_COMPONENT_ID")
        .expect("TIMELINE_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/timeline-manager")
}

fn add_worker(worker_id: String) -> Option<Worker> {
    let worker_hash = hash(worker_id.as_str());
    println!("Adding worker with hash: {}", worker_hash);
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.workers_count += 1;
        let worker = Worker {
            id: s.workers_count.to_string(),
        };
        match s.workers.insert(worker_hash, worker.clone()) {
            Some(_) => None,
            None => Some(worker),
        }
    })
    // TODO: Rebalance workers data
}

#[allow(unused)]
fn remove_worker(worker_id: String) -> bool {
    let worker_hash = hash(&worker_id);
    println!("Removing worker with hash: {}", worker_hash);
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        if s.workers.remove(&worker_hash).is_some() {
            s.workers_count -= 1;
            true
        } else {
            false
        }
    })
    // TODO: Rebalance workers data
}

fn get_responsible_worker(key: String) -> Option<Worker> {
    let worker_hash = hash(key.as_str());
    println!(
        "Getting responsible worker for key: {} (hash: {})",
        key, worker_hash
    );
    STATE.with(|state| {
        let s = state.borrow();
        s.workers.get(&worker_hash).cloned()
    })
}

fn _post_tweet(user_id: String, content: String) -> Result<String, ()> {
    println!(
        "Posting tweet for user with id: {} with content: {}",
        user_id.clone(),
        content
    );

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(),
    });

    api.blocking_post_tweet(user_id.as_str(), content.as_str())
}

fn get_tweets(user_id: String) -> Result<Vec<String>, ()> {
    println!("Getting tweets for user with id: {}", user_id.clone());

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(),
    });

    api.blocking_get_user_tweets(user_id.as_str())
}

fn _update_timeline(user_id: String, tweet_id: String) -> Result<bool, ()> {
    println!(
        "Updating timeline for user with id: {} with tweet with id: {}",
        user_id.clone(),
        tweet_id.clone()
    );

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TimelineApi::new(&Uri {
        value: get_timeline_worker_urn(),
    });

    api.blocking_update_timeline(user_id.as_str(), tweet_id.as_str())
}

fn get_timeline(user_id: String) -> Result<Vec<String>, ()> {
    println!("Getting timeline for user with id: {}", user_id.clone());

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    use bindings::golem::rpc::types::Uri;

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
    use bindings::golem::rpc::types::Uri;

    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_username(user_id.as_str())
}

fn get_followers(urn: String, user_id: String) -> Result<Vec<String>, ()> {
    println!(
        "Calling {} to get followers for user with id: {}",
        urn.clone(),
        user_id.clone()
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_followers(user_id.as_str())
}

fn get_following(urn: String, user_id: String) -> Result<Vec<String>, ()> {
    println!(
        "Calling {} to get following for user with id: {}",
        urn.clone(),
        user_id.clone()
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

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

// add a test for add_worker
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_worker_urn() {
        let user_id = "test_user_id".to_string();
        let component_id = "test_component_id".to_string();
        std::env::set_var("USER_MANAGER_COMPONENT_ID", component_id.clone());
        assert_eq!(
            get_user_worker_urn(user_id.clone()),
            format!("urn:worker:{}/user-manager-1", component_id)
        );
    }

    #[test]
    fn test_get_responsible_worker() {
        let worker_id = "test_worker_id".to_string();
        add_worker(worker_id.clone());
        assert_eq!(get_responsible_worker(worker_id.clone()).unwrap().id, "1");
    }

    #[test]
    fn test_add_worker() {
        let worker_id = "test_worker_id".to_string();
        assert_eq!(
            add_worker(worker_id.clone()),
            Some(Worker {
                id: "1".to_string()
            })
        );
        assert_eq!(add_worker(worker_id), None);
    }

    #[test]
    fn test_remove_worker() {
        let worker_id = "test_worker_id".to_string();
        add_worker(worker_id.clone());
        assert!(remove_worker(worker_id.clone()));
        assert!(!remove_worker(worker_id));
    }
}
