#[allow(warnings)]
mod bindings;

use bindings::exports::component::user_management::api::*;

struct Component;

impl Guest for Component {
    // Create stub implementation for user_management.wit

    // create-user: func(username: string) -> result<string>;
    fn create_user(username: String) -> Result<String, ()> {
        println!("Creating user with username: {}", username);
        Ok("".to_string())
    }

    // update-profile-picture: func(user-id: string, picture-data: list<u8>) -> result<bool>;
    fn update_profile_picture(user_id: String, picture_data: Vec<u8>) -> Result<bool, ()> {
        println!("Updating profile picture for user with id: {}", user_id);
        Ok(true)
    }

    // get-profile-picture: func(user-id: string) -> result<list<u8>>;
    fn get_profile_picture(user_id: String) -> Result<Vec<u8>, ()> {
        println!("Getting profile picture for user with id: {}", user_id);
        Ok(vec![])
    }

    // follow-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn follow_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now following user with id: {}",
            user_id, target_user_id
        );
        Ok(true)
    }

    // unfollow-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn unfollow_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now unfollowing user with id: {}",
            user_id, target_user_id
        );
        Ok(true)
    }

    // get-followers: func(user-id: string) -> result<list<string>>;
    fn get_followers(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting followers for user with id: {}", user_id);
        Ok(vec![])
    }

    // get-following: func(user-id: string) -> result<list<string>>;
    fn get_following(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting following for user with id: {}", user_id);
        Ok(vec![])
    }
}

bindings::export!(Component with_types_in bindings);
