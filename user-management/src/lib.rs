#[allow(warnings)]
mod bindings;

use bindings::exports::component::user_management::user_api::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UserId(String);

#[derive(Debug)]
struct User {
    username: String,
    profile_picture: Vec<u8>,
    followers: Vec<UserId>,
    following: Vec<UserId>,
}

impl User {
    fn new(username: String) -> Self {
        User {
            username,
            profile_picture: vec![],
            followers: vec![],
            following: vec![],
        }
    }
}

struct State {
    users_count: u64,
    users: BTreeMap<UserId, User>,
}

impl State {
    fn new() -> Self {
        State {
            users_count: 0,
            users: BTreeMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

struct Component;

impl Guest for Component {
    // create-user: func(username: string) -> result<string>;
    fn create_user(user_id: String, username: String) -> Result<String, ()> {
        println!(
            "Creating user with username: {} and user id: {}",
            username, user_id
        );
        let user_id = UserId(user_id);
        let exists = STATE.with_borrow(|state| state.users.contains_key(&user_id));
        if exists {
            Err(())
        } else {
            STATE.with_borrow_mut(|state| {
                let user = User::new(username);
                match state.users.insert(user_id.clone(), user) {
                    None => {
                        state.users_count += 1;
                        Ok(user_id.0)
                    }
                    _ => Err(()),
                }
            })
        }
    }

    // update-profile-picture: func(user-id: string, picture-data: list<u8>) -> result<bool>;
    fn update_profile_picture(user_id: String, picture_data: Vec<u8>) -> Result<bool, ()> {
        println!("Updating profile picture for user with id: {}", user_id);
        STATE.with_borrow_mut(|state| {
            if let Some(user) = state.users.get_mut(&UserId(user_id)) {
                user.profile_picture = picture_data;
                Ok(true)
            } else {
                Err(())
            }
        })
    }

    // get-profile-picture: func(user-id: string) -> result<list<u8>>;
    fn get_profile_picture(user_id: String) -> Result<Vec<u8>, ()> {
        println!("Getting profile picture for user with id: {}", user_id);
        STATE.with_borrow(|state| {
            if let Some(user) = state.users.get(&UserId(user_id)) {
                Ok(user.profile_picture.clone())
            } else {
                Err(())
            }
        })
    }

    // follow-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn follow_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now following user with id: {}",
            user_id, target_user_id
        );
        let user_id = UserId(user_id);
        let target_user_id = UserId(target_user_id);
        let exists = STATE.with_borrow(|state| {
            state
                .users
                .get(&user_id)
                .map(|user| user.following.contains(&target_user_id))
                .unwrap_or(false)
        });
        if exists {
            Ok(false)
        } else {
            STATE.with_borrow_mut(|state| {
                if let Some(user) = state.users.get_mut(&user_id) {
                    user.following.push(target_user_id.clone());
                    Ok(true)
                } else {
                    Err(())
                }
            })
        }
    }

    // unfollow-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn unfollow_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now unfollowing user with id: {}",
            user_id, target_user_id
        );
        let user_id = UserId(user_id);
        let target_user_id = UserId(target_user_id);
        let exists = STATE.with_borrow(|state| {
            state
                .users
                .get(&user_id)
                .map(|user| user.following.contains(&target_user_id))
                .unwrap_or(false)
        });
        if exists {
            STATE.with_borrow_mut(|state| {
                if let Some(user) = state.users.get_mut(&user_id) {
                    user.following.retain(|id| id != &target_user_id);
                    Ok(true)
                } else {
                    Err(())
                }
            })
        } else {
            Ok(false)
        }
    }

    // followed-by-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn followed_by_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now being followed by user with id: {}",
            user_id, target_user_id
        );
        let user_id = UserId(user_id);
        let target_user_id = UserId(target_user_id);
        let exists = STATE.with_borrow(|state| {
            state
                .users
                .get(&user_id)
                .map(|user| user.followers.contains(&target_user_id))
                .unwrap_or(false)
        });
        if exists {
            Ok(false)
        } else {
            STATE.with_borrow_mut(|state| {
                if let Some(user) = state.users.get_mut(&user_id) {
                    user.followers.push(target_user_id.clone());
                    Ok(true)
                } else {
                    Err(())
                }
            })
        }
    }

    // unfollowed-by-user: func(user-id: string, target-user-id: string) -> result<bool>;
    fn unfollowed_by_user(user_id: String, target_user_id: String) -> Result<bool, ()> {
        println!(
            "User with id: {} is now being unfollowed by user with id: {}",
            user_id, target_user_id
        );
        let user_id = UserId(user_id);
        let target_user_id = UserId(target_user_id);
        let exists = STATE.with_borrow(|state| {
            state
                .users
                .get(&user_id)
                .map(|user| user.followers.contains(&target_user_id))
                .unwrap_or(false)
        });
        if exists {
            STATE.with_borrow_mut(|state| {
                if let Some(user) = state.users.get_mut(&user_id) {
                    user.followers.retain(|id| id != &target_user_id);
                    Ok(true)
                } else {
                    Err(())
                }
            })
        } else {
            Ok(false)
        }
    }

    // get-username: func(user-id: string) -> result<string>;
    fn get_username(user_id: String) -> Result<String, ()> {
        println!("Getting username for user with id: {}", user_id);
        STATE.with_borrow(|state| {
            if let Some(user) = state.users.get(&UserId(user_id)) {
                Ok(user.username.clone())
            } else {
                Err(())
            }
        })
    }

    // get-followers: func(user-id: string) -> result<list<string>>;
    fn get_followers(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting followers for user with id: {}", user_id);
        STATE.with_borrow(|state| {
            if let Some(user) = state.users.get(&UserId(user_id)) {
                Ok(user.followers.iter().map(|id| id.0.clone()).collect())
            } else {
                Err(())
            }
        })
    }

    // get-following: func(user-id: string) -> result<list<string>>;
    fn get_following(user_id: String) -> Result<Vec<String>, ()> {
        println!("Getting following for user with id: {}", user_id);
        STATE.with_borrow(|state| {
            if let Some(user) = state.users.get(&UserId(user_id)) {
                Ok(user.following.iter().map(|id| id.0.clone()).collect())
            } else {
                Err(())
            }
        })
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_user(user_id: &'static str, username: &'static str) -> String {
        Component::create_user(user_id.to_string(), username.to_string()).unwrap()
    }
    fn setup_following(user_id: &'static str, target_user_id: &'static str, partial: bool) {
        Component::follow_user(user_id.to_string(), target_user_id.to_string()).unwrap();
        if !partial {
            Component::followed_by_user(target_user_id.to_string(), user_id.to_string()).unwrap();
        }
    }

    #[test]
    fn test_create_user() {
        let user_id = "1".to_string();
        let username = "user1".to_string();
        let result = Component::create_user(user_id.clone(), username.clone());
        assert_eq!(result, Ok(user_id));
    }

    #[test]
    fn test_create_user_already_exists() {
        let user_id = setup_user("1", "user1");
        let username = "user1".to_string();
        let result = Component::create_user(user_id.clone(), username.clone());
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_follow_user() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        let result = Component::follow_user(user_id.clone(), target_user_id.clone());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_follow_already_exists() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        setup_following("1", "2", true);
        let result = Component::follow_user(user_id.clone(), target_user_id.clone());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_unfollow_user() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        setup_following("1", "2", true);
        let result = Component::unfollow_user(user_id.clone(), target_user_id.clone());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_unfollow_not_exists() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        let result = Component::unfollow_user(user_id.clone(), target_user_id.clone());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_followed_by_user() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        let result = Component::followed_by_user(target_user_id.clone(), user_id.clone());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_followed_by_already_exists() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        setup_following("1", "2", false);
        let result = Component::followed_by_user(target_user_id.clone(), user_id.clone());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_unfollowed_by_user() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        setup_following("1", "2", false);
        let result = Component::unfollowed_by_user(target_user_id.clone(), user_id.clone());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_unfollowed_by_not_exists() {
        let user_id = setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        let result = Component::unfollowed_by_user(target_user_id.clone(), user_id.clone());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_get_username() {
        let user_id = setup_user("1", "user1");
        let result = Component::get_username(user_id.clone());
        assert_eq!(result, Ok("user1".to_string()));
    }

    #[test]
    fn test_get_followers() {
        setup_user("1", "user1");
        let target_user_id = setup_user("2", "user2");
        setup_following("1", "2", false);
        let result = Component::get_followers(target_user_id.clone());
        assert_eq!(result, Ok(vec!["1".to_string()]));
    }

    #[test]
    fn test_get_following() {
        let user_id = setup_user("1", "user1");
        setup_user("2", "user2");
        setup_following("1", "2", true);
        let result = Component::get_following(user_id.clone());
        assert_eq!(result, Ok(vec!["2".to_string()]));
    }

    #[test]
    fn test_update_profile_picture() {
        let user_id = setup_user("1", "user1");
        let picture_data = vec![1, 2, 3];
        let result_update =
            Component::update_profile_picture(user_id.clone(), picture_data.clone());
        let result_get = Component::get_profile_picture(user_id.clone());
        assert_eq!(result_update, Ok(true));
        assert_eq!(result_get, Ok(picture_data));
    }
}
