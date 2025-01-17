use crate::bindings::exports::component::golem_x_exports::tweet_api::PostedTweet;
use crate::bindings::exports::component::golem_x_exports::user_api::Username;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct State {
    pub picture: Vec<u8>,
    pub followers: HashSet<Username>,
    pub followings: HashSet<Username>,
    pub tweets: Vec<PostedTweet>,
}

impl State {
    pub fn new() -> Self {
        Self {
            picture: Vec::new(),
            followers: HashSet::new(),
            followings: HashSet::new(),
            tweets: Vec::new(),
        }
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::new());
    pub static USERNAME: Username = std::env::var("GOLEM_WORKER_NAME")
        .expect("GOLEM_WORKER_NAME not set")
        .strip_prefix("user-")
        .expect("Invalid GOLEM_WORKER_NAME")
        .to_string();
}

pub fn get_username() -> Username {
    USERNAME.with(|u| u.clone())
}

pub fn update<R, F: FnOnce(&mut State) -> R>(f: F) -> R {
    STATE.with_borrow_mut(f)
}

pub fn get<R, F: FnOnce(&State) -> R>(f: F) -> R {
    STATE.with_borrow(f)
}
