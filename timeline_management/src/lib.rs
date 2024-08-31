#[allow(warnings)]
mod bindings;

use bindings::exports::component::timeline_management::api::*;

struct Component;

impl Guest for Component {
    //  update-timeline: func(user-id: string, tweet-id: string, author-id: string) -> result<bool>;
    fn update_timeline(user_id: String, tweet_id: String, author_id: String) -> Result<bool, ()> {
        println!(
            "Updating timeline for user with id: {} with tweet with id: {} from author with id: {}",
            user_id, tweet_id, author_id
        );
        Ok(true)
    }

    //  get-timeline: func(user-id: string) -> result<list<string>>;
    fn get_timeline(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting timeline for user with id: {}", user_id);
        Ok(vec![])
    }

    //  refresh-timeline: func(user-id: string, followed-user-id: string, action: action) -> result<bool>;
    fn refresh_timeline(
        user_id: String,
        followed_user_id: String,
        action: Action,
    ) -> Result<bool, ()> {
        println!(
            "Refreshing timeline for user with id: {} with tweets from user with id: {} using action: {:?}",
            user_id, followed_user_id, action
        );
        Ok(true)
    }
}

bindings::export!(Component with_types_in bindings);
