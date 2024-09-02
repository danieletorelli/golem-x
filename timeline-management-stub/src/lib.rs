#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct TimelineApi {
    rpc: WasmRpc,
}
impl TimelineApi {}
pub struct FutureUpdateTimelineResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetTimelineResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::Guest
for Component {
    type TimelineApi = crate::TimelineApi;
    type FutureUpdateTimelineResult = crate::FutureUpdateTimelineResult;
    type FutureGetTimelineResult = crate::FutureGetTimelineResult;
}
impl crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::GuestFutureUpdateTimelineResult
for FutureUpdateTimelineResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<bool, ()>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:timeline-management/timeline-api.{update-timeline}"
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
                                    .bool()
                                    .expect("bool not found"),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::GuestFutureGetTimelineResult
for FutureGetTimelineResult {
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
            Vec<
                crate::bindings::component::timeline_management::timeline_api::TimelineTweet,
            >,
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
                            "component:timeline-management/timeline-api.{get-timeline}"
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
                                        crate::bindings::component::timeline_management::timeline_api::TimelineTweet {
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
                                            timestamp: record
                                                .field(2usize)
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
impl crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::GuestTimelineApi
for TimelineApi {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_update_timeline(
        &self,
        user_id: u64,
        tweet_id: u64,
        author_id: u64,
        timestamp: i64,
        action: crate::bindings::component::timeline_management::timeline_api::TimelineAction,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:timeline-management/timeline-api.{update-timeline}",
                &[
                    WitValue::builder().u64(user_id),
                    WitValue::builder().u64(tweet_id),
                    WitValue::builder().u64(author_id),
                    WitValue::builder().s64(timestamp),
                    WitValue::builder()
                        .enum_value(
                            match action {
                                crate::bindings::component::timeline_management::timeline_api::TimelineAction::Insert => {
                                    0u32
                                }
                                crate::bindings::component::timeline_management::timeline_api::TimelineAction::Remove => {
                                    1u32
                                }
                            },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:timeline-management/timeline-api.{update-timeline}"
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
                            .bool()
                            .expect("bool not found"),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn update_timeline(
        &self,
        user_id: u64,
        tweet_id: u64,
        author_id: u64,
        timestamp: i64,
        action: crate::bindings::component::timeline_management::timeline_api::TimelineAction,
    ) -> crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureUpdateTimelineResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:timeline-management/timeline-api.{update-timeline}",
                &[
                    WitValue::builder().u64(user_id),
                    WitValue::builder().u64(tweet_id),
                    WitValue::builder().u64(author_id),
                    WitValue::builder().s64(timestamp),
                    WitValue::builder()
                        .enum_value(
                            match action {
                                crate::bindings::component::timeline_management::timeline_api::TimelineAction::Insert => {
                                    0u32
                                }
                                crate::bindings::component::timeline_management::timeline_api::TimelineAction::Remove => {
                                    1u32
                                }
                            },
                        ),
                ],
            );
        crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureUpdateTimelineResult::new(FutureUpdateTimelineResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_timeline(
        &self,
        user_id: u64,
    ) -> Result<
        Vec<
            crate::bindings::component::timeline_management::timeline_api::TimelineTweet,
        >,
        (),
    > {
        let result = self
            .rpc
            .invoke_and_await(
                "component:timeline-management/timeline-api.{get-timeline}",
                &[WitValue::builder().u64(user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:timeline-management/timeline-api.{get-timeline}"
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
                                crate::bindings::component::timeline_management::timeline_api::TimelineTweet {
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
                                    timestamp: record
                                        .field(2usize)
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
    fn get_timeline(
        &self,
        user_id: u64,
    ) -> crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureGetTimelineResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:timeline-management/timeline-api.{get-timeline}",
                &[WitValue::builder().u64(user_id)],
            );
        crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureGetTimelineResult::new(FutureGetTimelineResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
