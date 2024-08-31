#[allow(warnings)]
mod bindings;

use bindings::exports::component::router::api::*;

struct Component;

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

    //  add-worker: func(worker-id: string) -> result<bool>;
    fn add_worker(worker_id: String) -> Result<bool, ()> {
        println!("Adding worker with id: {}", worker_id);
        Ok(true)
    }

    //  remove-worker: func(worker-id: string) -> result<bool>;
    fn remove_worker(worker_id: String) -> Result<bool, ()> {
        println!("Removing worker with id: {}", worker_id);
        Ok(true)
    }

    //  get-responsible-worker: func(key: string) -> result<string>;
    fn get_responsible_worker(key: String) -> Result<String, ()> {
        println!("Getting responsible worker for key: {}", key);
        Ok("".to_string())
    }

    // route-query: func(query-type: query-type, key: string, data: string) -> result<string>;
    fn route_query(query_type: QueryType, key: String, data: String) -> Result<String, ()> {
        println!(
            "Routing query of type: {:?} for key: {} with data: {}",
            query_type, key, data
        );
        Ok("".to_string())
    }
}

bindings::export!(Component with_types_in bindings);
