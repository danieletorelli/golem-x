use crate::bindings::exports::component::golem_x_interface::timeline_api::TimelineTweet;
use std::cell::RefCell;

thread_local! {
    pub static TIMELINE_CACHE: RefCell<Option<Vec<TimelineTweet>>> = const { RefCell::new(None) };
}

pub fn get() -> Option<Vec<TimelineTweet>> {
    TIMELINE_CACHE.with(|c| c.borrow().clone())
}

pub fn update(timeline: Vec<TimelineTweet>) {
    println!("Caching timeline");
    TIMELINE_CACHE.with_borrow_mut(|c| {
        *c = Some(timeline);
    });
}

pub fn invalidate() {
    println!("Invalidating timeline cache");
    TIMELINE_CACHE.with_borrow_mut(|c| {
        *c = None;
    });
}
