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
        println!("Creating user with username: {}", username);
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
            state.users.contains_key(&user_id) && state.users.contains_key(&target_user_id)
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
            state.users.contains_key(&user_id) && state.users.contains_key(&target_user_id)
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
            state.users.contains_key(&user_id) && state.users.contains_key(&target_user_id)
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
            state.users.contains_key(&user_id) && state.users.contains_key(&target_user_id)
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
