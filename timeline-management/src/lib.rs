#[allow(warnings)]
mod bindings;

use bindings::exports::component::timeline_management::timeline_api::*;

struct Component;

impl Guest for Component {
    //  update-timeline: func(user-id: string, tweet-id: string, author-id: string) -> result<bool>;
    fn update_timeline(user_id: String, tweet_id: String) -> Result<bool, ()> {
        println!(
            "Updating timeline for user with id: {} with tweet id: {}",
            user_id, tweet_id
        );
        Ok(true)
    }

    //  get-timeline: func(user-id: string) -> result<list<string>>;
    fn get_timeline(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting timeline for user with id: {}", user_id);
        Ok(vec![])
    }
}

bindings::export!(Component with_types_in bindings);
