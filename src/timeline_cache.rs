use crate::bindings::exports::component::golem_x_exports::timeline_api::TimelineTweet;
use std::cell::RefCell;

thread_local! {
    static TIMELINE_CACHE: RefCell<Option<Vec<TimelineTweet>>> = const { RefCell::new(None) };
}

#[inline(always)]
pub fn get() -> Option<Vec<TimelineTweet>> {
    TIMELINE_CACHE.with_borrow(|c| c.clone())
}

#[inline(always)]
pub fn update(timeline: Vec<TimelineTweet>) {
    println!("Caching timeline");
    TIMELINE_CACHE.with_borrow_mut(|c| *c = Some(timeline));
}

#[inline(always)]
pub fn invalidate() {
    println!("Invalidating timeline cache");
    TIMELINE_CACHE.with_borrow_mut(|c| *c = None);
}
