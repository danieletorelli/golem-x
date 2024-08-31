#[allow(warnings)]
mod bindings;

use bindings::exports::component::tweet_management::api::*;

struct Component;

impl Guest for Component {
    // post-tweet: func(user-id: string, content: string) -> result<string>;
    fn post_tweet(user_id: String, content: String) -> Result<String, ()> {
        println!("User with id: {} posted tweet: {}", user_id, content);
        Ok("".to_string())
    }

    //  get-user-tweets: func(user-id: string) -> result<list<string>>;
    fn get_user_tweets(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting tweets for user with id: {}", user_id);
        Ok(vec![])
    }
}

bindings::export!(Component with_types_in bindings);
