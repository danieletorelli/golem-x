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
    fn get(&self) -> Option<Result<Vec<String>, ()>> {
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
                                        item.string().expect("string not found").to_string()
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
        user_id: String,
        tweet_id: String,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:timeline-management/timeline-api.{update-timeline}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&tweet_id),
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
        user_id: String,
        tweet_id: String,
    ) -> crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureUpdateTimelineResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:timeline-management/timeline-api.{update-timeline}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&tweet_id),
                ],
            );
        crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureUpdateTimelineResult::new(FutureUpdateTimelineResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_timeline(&self, user_id: String) -> Result<Vec<String>, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:timeline-management/timeline-api.{get-timeline}",
                &[WitValue::builder().string(&user_id)],
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
                                item.string().expect("string not found").to_string()
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
        user_id: String,
    ) -> crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureGetTimelineResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:timeline-management/timeline-api.{get-timeline}",
                &[WitValue::builder().string(&user_id)],
            );
        crate::bindings::exports::component::timeline_management_stub::stub_timeline_management::FutureGetTimelineResult::new(FutureGetTimelineResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
