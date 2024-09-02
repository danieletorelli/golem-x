#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct TweetApi {
    rpc: WasmRpc,
}
impl TweetApi {}
pub struct FuturePostTweetResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetUserTweetsResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetSpecificTweetsResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::Guest
for Component {
    type TweetApi = crate::TweetApi;
    type FuturePostTweetResult = crate::FuturePostTweetResult;
    type FutureGetUserTweetsResult = crate::FutureGetUserTweetsResult;
    type FutureGetSpecificTweetsResult = crate::FutureGetSpecificTweetsResult;
}
impl crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::GuestFuturePostTweetResult
for FuturePostTweetResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(
        &self,
    ) -> Option<
        Result<crate::bindings::component::tweet_management::tweet_api::PostedTweet, ()>,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:tweet-management/tweet-api.{post-tweet}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok({
                                let record = ok_value.expect("result ok value not found");
                                crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                                    tweet_id: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    author_id: record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    content: record
                                        .field(2usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    timestamp: record
                                        .field(3usize)
                                        .expect("record field not found")
                                        .s64()
                                        .expect("i64 not found"),
                                }
                            })
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::GuestFutureGetUserTweetsResult
for FutureGetUserTweetsResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(
        &self,
    ) -> Option<
        Result<
            Vec<crate::bindings::component::tweet_management::tweet_api::PostedTweet>,
            (),
        >,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:tweet-management/tweet-api.{get-user-tweets}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .list_elements(|item| {
                                        let record = item;
                                        crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                                            tweet_id: record
                                                .field(0usize)
                                                .expect("record field not found")
                                                .u64()
                                                .expect("u64 not found"),
                                            author_id: record
                                                .field(1usize)
                                                .expect("record field not found")
                                                .u64()
                                                .expect("u64 not found"),
                                            content: record
                                                .field(2usize)
                                                .expect("record field not found")
                                                .string()
                                                .expect("string not found")
                                                .to_string(),
                                            timestamp: record
                                                .field(3usize)
                                                .expect("record field not found")
                                                .s64()
                                                .expect("i64 not found"),
                                        }
                                    })
                                    .expect("list not found"),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::GuestFutureGetSpecificTweetsResult
for FutureGetSpecificTweetsResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(
        &self,
    ) -> Option<
        Result<
            Vec<crate::bindings::component::tweet_management::tweet_api::PostedTweet>,
            (),
        >,
    > {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:tweet-management/tweet-api.{get-specific-tweets}"
                        ),
                    );
                ({
                    let result = result
                        .tuple_element(0)
                        .expect("tuple not found")
                        .result()
                        .expect("result not found");
                    match result {
                        Ok(ok_value) => {
                            Ok(
                                ok_value
                                    .expect("result ok value not found")
                                    .list_elements(|item| {
                                        let record = item;
                                        crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                                            tweet_id: record
                                                .field(0usize)
                                                .expect("record field not found")
                                                .u64()
                                                .expect("u64 not found"),
                                            author_id: record
                                                .field(1usize)
                                                .expect("record field not found")
                                                .u64()
                                                .expect("u64 not found"),
                                            content: record
                                                .field(2usize)
                                                .expect("record field not found")
                                                .string()
                                                .expect("string not found")
                                                .to_string(),
                                            timestamp: record
                                                .field(3usize)
                                                .expect("record field not found")
                                                .s64()
                                                .expect("i64 not found"),
                                        }
                                    })
                                    .expect("list not found"),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::GuestTweetApi
for TweetApi {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_post_tweet(
        &self,
        user_id: u64,
        content: String,
    ) -> Result<
        crate::bindings::component::tweet_management::tweet_api::PostedTweet,
        (),
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "component:tweet-management/tweet-api.{post-tweet}",
                &[WitValue::builder().u64(user_id), WitValue::builder().string(&content)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:tweet-management/tweet-api.{post-tweet}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok({
                        let record = ok_value.expect("result ok value not found");
                        crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                            tweet_id: record
                                .field(0usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            author_id: record
                                .field(1usize)
                                .expect("record field not found")
                                .u64()
                                .expect("u64 not found"),
                            content: record
                                .field(2usize)
                                .expect("record field not found")
                                .string()
                                .expect("string not found")
                                .to_string(),
                            timestamp: record
                                .field(3usize)
                                .expect("record field not found")
                                .s64()
                                .expect("i64 not found"),
                        }
                    })
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn post_tweet(
        &self,
        user_id: u64,
        content: String,
    ) -> crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FuturePostTweetResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:tweet-management/tweet-api.{post-tweet}",
                &[WitValue::builder().u64(user_id), WitValue::builder().string(&content)],
            );
        crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FuturePostTweetResult::new(FuturePostTweetResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_user_tweets(
        &self,
        user_id: u64,
    ) -> Result<
        Vec<crate::bindings::component::tweet_management::tweet_api::PostedTweet>,
        (),
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "component:tweet-management/tweet-api.{get-user-tweets}",
                &[WitValue::builder().u64(user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:tweet-management/tweet-api.{get-user-tweets}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .list_elements(|item| {
                                let record = item;
                                crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                                    tweet_id: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    author_id: record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    content: record
                                        .field(2usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    timestamp: record
                                        .field(3usize)
                                        .expect("record field not found")
                                        .s64()
                                        .expect("i64 not found"),
                                }
                            })
                            .expect("list not found"),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn get_user_tweets(
        &self,
        user_id: u64,
    ) -> crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FutureGetUserTweetsResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:tweet-management/tweet-api.{get-user-tweets}",
                &[WitValue::builder().u64(user_id)],
            );
        crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FutureGetUserTweetsResult::new(FutureGetUserTweetsResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_specific_tweets(
        &self,
        user_id: u64,
        tweet_ids: Vec<u64>,
    ) -> Result<
        Vec<crate::bindings::component::tweet_management::tweet_api::PostedTweet>,
        (),
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "component:tweet-management/tweet-api.{get-specific-tweets}",
                &[
                    WitValue::builder().u64(user_id),
                    WitValue::builder()
                        .list_fn(
                            &tweet_ids,
                            |item, item_builder| { item_builder.u64(*item) },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:tweet-management/tweet-api.{get-specific-tweets}"
                ),
            );
        ({
            let result = result
                .tuple_element(0)
                .expect("tuple not found")
                .result()
                .expect("result not found");
            match result {
                Ok(ok_value) => {
                    Ok(
                        ok_value
                            .expect("result ok value not found")
                            .list_elements(|item| {
                                let record = item;
                                crate::bindings::component::tweet_management::tweet_api::PostedTweet {
                                    tweet_id: record
                                        .field(0usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    author_id: record
                                        .field(1usize)
                                        .expect("record field not found")
                                        .u64()
                                        .expect("u64 not found"),
                                    content: record
                                        .field(2usize)
                                        .expect("record field not found")
                                        .string()
                                        .expect("string not found")
                                        .to_string(),
                                    timestamp: record
                                        .field(3usize)
                                        .expect("record field not found")
                                        .s64()
                                        .expect("i64 not found"),
                                }
                            })
                            .expect("list not found"),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn get_specific_tweets(
        &self,
        user_id: u64,
        tweet_ids: Vec<u64>,
    ) -> crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FutureGetSpecificTweetsResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:tweet-management/tweet-api.{get-specific-tweets}",
                &[
                    WitValue::builder().u64(user_id),
                    WitValue::builder()
                        .list_fn(
                            &tweet_ids,
                            |item, item_builder| { item_builder.u64(*item) },
                        ),
                ],
            );
        crate::bindings::exports::component::tweet_management_stub::stub_tweet_management::FutureGetSpecificTweetsResult::new(FutureGetSpecificTweetsResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
