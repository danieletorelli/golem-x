#[allow(warnings)]
mod bindings;

use bindings::component::timeline_management_stub::stub_timeline_management::TimelineAction;
use bindings::component::tweet_management_stub::stub_tweet_management::PostedTweet;
use bindings::exports::component::router::router_api::*;
use blake2::Digest;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WorkerId(u8);

#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
struct WorkerHash(u8);

#[derive(Debug, Clone, PartialEq)]
struct Worker {
    id: WorkerId,
    hash: WorkerHash,
}

struct State {
    next_user_id: u64,
    workers_count: u8,
    workers: BTreeMap<WorkerHash, Worker>,
}

impl State {
    fn new() -> Self {
        State {
            next_user_id: 0,
            workers_count: 0,
            workers: BTreeMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

struct Component;

fn hash(input: u64) -> WorkerHash {
    let mut hasher = blake2::Blake2s256::new();
    hasher.update(input.to_le_bytes());
    let result = hasher.finalize();
    // Convert the first byte of the hash into u64
    let bytes: [u8; 1] = result[..1].try_into().expect("slice with incorrect length");
    WorkerHash(u8::from_le_bytes(bytes))
}

fn get_user_worker_urn(user_id: u64) -> String {
    let worker_id = get_responsible_worker(user_id);
    let component_id =
        std::env::var("USER_MANAGER_COMPONENT_ID").expect("USER_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/user-manager-{}", worker_id.0)
}
fn get_tweet_worker_urn(user_id: u64) -> String {
    let worker_id = get_responsible_worker(user_id);
    let component_id =
        std::env::var("TWEET_MANAGER_COMPONENT_ID").expect("TWEET_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/tweet-manager-{}", worker_id.0)
}
fn get_timeline_worker_urn(user_id: u64) -> String {
    let worker_id = get_responsible_worker(user_id);
    let component_id = std::env::var("TIMELINE_MANAGER_COMPONENT_ID")
        .expect("TIMELINE_MANAGER_COMPONENT_ID not set");
    format!("urn:worker:{component_id}/timeline-manager-{}", worker_id.0)
}

fn add_worker(worker_id: u64) -> Option<Worker> {
    let worker_hash = hash(worker_id);
    println!("Adding worker with hash: {}", worker_hash.0);
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let worker = Worker {
            id: WorkerId(s.workers_count + 1),
            hash: worker_hash,
        };
        match s.workers.insert(worker_hash, worker.clone()) {
            None => {
                s.workers_count += 1;
                Some(worker)
            }
            Some(_) => None,
        }
    })
}

#[allow(unused)]
fn remove_worker(worker_id: u64) -> bool {
    let worker_hash = hash(worker_id);
    println!("Removing worker with hash: {}", worker_hash.0);
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        match s.workers.remove(&worker_hash) {
            Some(_) => {
                s.workers_count -= 1;
                true
            }
            None => false,
        }
    })
}

fn get_responsible_worker(user_id: u64) -> WorkerId {
    let worker_hash = hash(user_id);
    println!(
        "Getting responsible worker for user: {} (hash: {})",
        user_id, worker_hash.0
    );
    let existing = STATE.with_borrow(|state| state.workers.get(&worker_hash).cloned());
    match existing {
        Some(worker) => worker,
        None => add_worker(user_id)
            .unwrap_or_else(|| panic!("Failed to add worker for user with id: {}", user_id)),
    }
    .id
}

fn register_user(username: String) -> Result<u64, ()> {
    println!("Registering new user with username: {}", username);

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let available_user_id = STATE.with_borrow(|state| state.next_user_id);

    let api = UserApi::new(&Uri {
        value: get_user_worker_urn(available_user_id),
    });

    match api.blocking_create_user(available_user_id, username.as_str()) {
        Ok(user_id) => {
            if user_id != available_user_id {
                return Err(());
            }
            STATE.with_borrow_mut(|state| state.next_user_id += 1);
            Ok(user_id)
        }
        Err(_) => Err(()),
    }
}

fn get_tweets(user_id: u64) -> Result<Vec<PostedTweet>, ()> {
    println!("Getting tweets for user with id: {}", user_id);

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(user_id),
    });

    api.blocking_get_user_tweets(user_id)
}

fn get_specific_tweets(user_id: u64, tweet_ids: Vec<u64>) -> Result<Vec<PostedTweet>, ()> {
    println!("Getting specific tweets for user with id: {}", user_id);

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(user_id),
    });

    api.blocking_get_specific_tweets(user_id, tweet_ids.as_slice())
}

fn update_timeline(
    user_id: u64,
    tweet_id: u64,
    author_id: u64,
    timestamp: i64,
    action: TimelineAction,
) -> Result<bool, ()> {
    println!(
        "Updating timeline for user with id: {} with tweet with id: {} and timestamp: {}",
        user_id, tweet_id, timestamp
    );

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TimelineApi::new(&Uri {
        value: get_timeline_worker_urn(user_id),
    });

    api.blocking_update_timeline(user_id, tweet_id, author_id, timestamp, action)
}

fn get_timeline(user_id: u64) -> Result<Vec<Tweet>, ()> {
    println!("Getting timeline for user with id: {}", user_id);

    use bindings::component::timeline_management_stub::stub_timeline_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = TimelineApi::new(&Uri {
        value: get_timeline_worker_urn(user_id),
    });

    api.blocking_get_timeline(user_id).and_then(|tweets| {
        // Group tweets by author id, fetch their content and return them sorted by timestamp
        let mut tweets_by_author: BTreeMap<u64, Vec<TimelineTweet>> = BTreeMap::new();
        for tweet in tweets {
            tweets_by_author
                .entry(tweet.author_id)
                .or_default()
                .push(tweet);
        }
        // Iterate over user ids and fetch their content
        let mut expanded_tweets: BTreeMap<u64, Vec<PostedTweet>> = BTreeMap::new();
        for (author_id, tweets) in tweets_by_author {
            let author_tweets = get_specific_tweets(
                author_id,
                tweets
                    .into_iter()
                    .map(|tweet| tweet.tweet_id)
                    .collect::<Vec<_>>(),
            )?;
            expanded_tweets.insert(author_id, author_tweets);
        }
        // Tweets of all users, sorted by timestamp
        let mut timeline: Vec<Tweet> = expanded_tweets
            .into_iter()
            .flat_map(|(author_id, tweets)| {
                tweets.into_iter().map(move |tweet| Tweet {
                    tweet_id: tweet.tweet_id,
                    author_id,
                    content: tweet.content,
                    timestamp: tweet.timestamp,
                })
            })
            .collect();
        timeline.sort_by_key(|tweet| tweet.timestamp);
        Ok(timeline)
    })
}

fn get_username(urn: String, user_id: u64) -> Result<String, ()> {
    println!(
        "Calling {} to get username for user with id: {}",
        urn.clone(),
        user_id
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_username(user_id)
}

fn get_followers(urn: String, user_id: u64) -> Result<Vec<u64>, ()> {
    println!(
        "Calling {} to get followers for user with id: {}",
        urn.clone(),
        user_id
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_followers(user_id)
}

fn get_following(urn: String, user_id: u64) -> Result<Vec<u64>, ()> {
    println!(
        "Calling {} to get following for user with id: {}",
        urn.clone(),
        user_id
    );

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let api = UserApi::new(&Uri { value: urn });

    api.blocking_get_following(user_id)
}

fn compose_user_profile(urn: String, user_id: u64) -> Result<UserProfile, String> {
    println!(
        "Calling {} to compose user profile for user {}",
        urn, user_id
    );

    let username_result = get_username(urn.clone(), user_id);
    let followers_result = get_followers(urn.clone(), user_id);
    let following_result = get_following(urn, user_id);

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

fn compose_tweets(urn: String, user_id: u64) -> Result<TweetData, ()> {
    println!("Calling {} to compose tweets for user {}", urn, user_id);

    get_tweets(user_id).map(|tweets| TweetData {
        user_id,
        tweets: tweets
            .into_iter()
            .map(|tweet| Tweet {
                tweet_id: tweet.tweet_id,
                author_id: tweet.author_id,
                content: tweet.content,
                timestamp: tweet.timestamp,
            })
            .collect(),
    })
}

fn compose_timeline(urn: String, user_id: u64) -> Result<Vec<TimelineData>, ()> {
    println!("Calling {} to compose timeline for user {}", urn, user_id);

    get_timeline(user_id).map(|tweets| {
        tweets
            .into_iter()
            .map(|tweet| TimelineData {
                author_id: tweet.author_id,
                tweet: tweet.clone(),
            })
            .collect()
    })
}

fn orchestrate_follow(user_id: u64, target_user_id: u64) -> Result<bool, ()> {
    println!(
        "Orchestrating follow for user {} and target user {}",
        user_id, target_user_id
    );

    let user_urn = get_user_worker_urn(user_id);
    let target_user_urn = get_user_worker_urn(target_user_id);

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let user_api = UserApi::new(&Uri { value: user_urn });
    let target_user_api = UserApi::new(&Uri {
        value: target_user_urn,
    });

    let follow_result = user_api.blocking_follow_user(user_id, target_user_id);
    let follow_back_result = target_user_api.blocking_followed_by_user(target_user_id, user_id);

    match (follow_result, follow_back_result) {
        (Ok(_), Ok(_)) => {
            // Add tweets of the target user to the user's timeline
            get_tweets(target_user_id)
                .and_then(|tweets| {
                    tweets
                        .into_iter()
                        .map(|tweet| {
                            update_timeline(
                                user_id,
                                tweet.tweet_id,
                                target_user_id,
                                tweet.timestamp,
                                TimelineAction::Insert,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .map(|_| true)
        }
        // TODO: Log inconsistent state, if happens
        (Err(_), Ok(_)) => {
            let _ = target_user_api.blocking_unfollowed_by_user(target_user_id, user_id);
            Err(())
        }
        (Ok(_), Err(_)) => {
            let _ = user_api.blocking_unfollow_user(user_id, target_user_id);
            Err(())
        }
        (Err(_), Err(_)) => Err(()),
    }
}

fn orchestrate_unfollow(user_id: u64, target_user_id: u64) -> Result<bool, ()> {
    println!(
        "Orchestrating unfollow for user {} and target user {}",
        user_id, target_user_id
    );

    let user_urn = get_user_worker_urn(user_id);
    let target_user_urn = get_user_worker_urn(target_user_id);

    use bindings::component::user_management_stub::stub_user_management::*;
    use bindings::golem::rpc::types::Uri;

    let user_api = UserApi::new(&Uri { value: user_urn });
    let target_user_api = UserApi::new(&Uri {
        value: target_user_urn,
    });

    let unfollow_result = user_api.blocking_unfollow_user(user_id, target_user_id);
    let unfollow_back_result = target_user_api.blocking_unfollowed_by_user(target_user_id, user_id);

    match (unfollow_result, unfollow_back_result) {
        (Ok(_), Ok(_)) => {
            // Remove tweets of the target user from the user's timeline
            get_tweets(target_user_id)
                .and_then(|tweets| {
                    tweets
                        .into_iter()
                        .map(|tweet| {
                            update_timeline(
                                user_id,
                                tweet.tweet_id,
                                target_user_id,
                                tweet.timestamp,
                                TimelineAction::Remove,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .map(|_| true)
        }
        // TODO: Log inconsistent state, if happens
        (Err(_), Ok(_)) => {
            let _ = target_user_api.blocking_followed_by_user(target_user_id, user_id);
            Err(())
        }
        (Ok(_), Err(_)) => {
            let _ = user_api.blocking_follow_user(user_id, target_user_id);
            Err(())
        }
        (Err(_), Err(_)) => Err(()),
    }
}

fn orchestrate_post_tweet(user_id: u64, content: String) -> Result<u64, ()> {
    println!(
        "Posting tweet for user with id: {} with content: {}",
        user_id, content
    );

    use bindings::component::tweet_management_stub::stub_tweet_management::*;
    use bindings::golem::rpc::types::Uri;

    let tweet_api = TweetApi::new(&Uri {
        value: get_tweet_worker_urn(user_id),
    });

    tweet_api
        .blocking_post_tweet(user_id, content.as_str())
        .and_then(|tweet| {
            get_followers(get_user_worker_urn(user_id), user_id).and_then(|followers| {
                followers
                    .into_iter()
                    .map(|follower_id| {
                        update_timeline(
                            follower_id,
                            tweet.tweet_id,
                            user_id,
                            tweet.timestamp,
                            TimelineAction::Insert,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|_| tweet.tweet_id)
            })
        })
}

impl Guest for Component {
    // route-action: func(action: action, user-id: string) -> router-action-response;
    fn route_action(action: Action, user_id: u64) -> RouterActionResponse {
        use bindings::exports::component::router::router_api::RouterActionResponse::*;
        println!("Routing action of type: {:?} for user: {}", action, user_id);
        match action {
            Action::Register(username) => register_user(username)
                .map(|_| Success)
                .unwrap_or_else(|_| Failure("Failed to register user".to_string())),
            Action::Follow(target_user_id) => orchestrate_follow(user_id, target_user_id)
                .map(|_| Success)
                .unwrap_or_else(|_| Failure("Failed to follow".to_string())),
            Action::Unfollow(target_user_id) => orchestrate_unfollow(user_id, target_user_id)
                .map(|_| Success)
                .unwrap_or_else(|_| Failure("Failed to unfollow".to_string())),
            Action::PostTweet(content) => orchestrate_post_tweet(user_id, content)
                .map(|_| Success)
                .unwrap_or_else(|_| Failure("Failed to post tweet".to_string())),
        }
    }

    // route-query: func(query-type: query-type, user-id: string) -> router-query-response;
    fn route_query(query_type: QueryType, user_id: u64) -> RouterQueryResponse {
        use bindings::exports::component::router::router_api::RouterQueryResponse::*;
        println!(
            "Routing query of type: {:?} for user: {}",
            query_type, user_id
        );
        match query_type {
            QueryType::UserProfile => {
                println!("Query type: UserProfile");
                let worker_urn = get_user_worker_urn(user_id);
                compose_user_profile(worker_urn, user_id)
                    .map(UserProfileResponse)
                    .unwrap_or_else(Failure)
            }
            QueryType::UserFollowers => {
                println!("Query type: UserFollowers");
                let worker_urn = get_user_worker_urn(user_id);
                get_followers(worker_urn, user_id)
                    .map(UserFollowersResponse)
                    .unwrap_or_else(|_| Failure("Failed to get followers".to_string()))
            }
            QueryType::UserFollowing => {
                println!("Query type: UserFollowing");
                let worker_urn = get_user_worker_urn(user_id);
                get_following(worker_urn, user_id)
                    .map(UserFollowingResponse)
                    .unwrap_or_else(|_| Failure("Failed to get following".to_string()))
            }
            QueryType::UserTweets => {
                println!("Query type: UserTweets");
                let worker_urn = get_tweet_worker_urn(user_id);
                compose_tweets(worker_urn, user_id)
                    .map(UserTweetsResponse)
                    .unwrap_or_else(|_| Failure("Failed to get tweets".to_string()))
            }
            QueryType::UserTimeline => {
                println!("Query type: UserTimeline");
                let worker_urn = get_timeline_worker_urn(user_id);
                compose_timeline(worker_urn, user_id)
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
        let user_id = 1;
        let component_id = "test_component_id".to_string();
        std::env::set_var("USER_MANAGER_COMPONENT_ID", component_id.clone());
        assert_eq!(
            get_user_worker_urn(user_id),
            format!("urn:worker:{}/user-manager-{}", component_id, user_id)
        );
    }

    #[test]
    fn test_get_responsible_worker() {
        let worker_id = 1;
        add_worker(worker_id);
        assert_eq!(get_responsible_worker(worker_id), WorkerId(1));
    }

    #[test]
    fn test_add_worker() {
        let worker_id = 1;
        assert_eq!(
            add_worker(worker_id),
            Some(Worker {
                id: WorkerId(1),
                hash: hash(worker_id)
            })
        );
        assert_eq!(add_worker(worker_id), None);
    }

    #[test]
    fn test_remove_worker() {
        let worker_id = 1;
        add_worker(worker_id);
        assert!(remove_worker(worker_id));
        assert!(!remove_worker(worker_id));
    }
}
