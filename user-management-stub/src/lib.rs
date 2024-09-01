#![allow(warnings)]
use golem_wasm_rpc::*;
#[allow(dead_code)]
mod bindings;
pub struct UserApi {
    rpc: WasmRpc,
}
impl UserApi {}
pub struct FutureCreateUserResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUpdateProfilePictureResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetProfilePictureResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureFollowUserResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUnfollowUserResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureFollowedByUserResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureUnfollowedByUserResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetUsernameResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetFollowersResult {
    pub future_invoke_result: FutureInvokeResult,
}
pub struct FutureGetFollowingResult {
    pub future_invoke_result: FutureInvokeResult,
}
struct Component;
impl crate::bindings::exports::component::user_management_stub::stub_user_management::Guest
for Component {
    type UserApi = crate::UserApi;
    type FutureCreateUserResult = crate::FutureCreateUserResult;
    type FutureUpdateProfilePictureResult = crate::FutureUpdateProfilePictureResult;
    type FutureGetProfilePictureResult = crate::FutureGetProfilePictureResult;
    type FutureFollowUserResult = crate::FutureFollowUserResult;
    type FutureUnfollowUserResult = crate::FutureUnfollowUserResult;
    type FutureFollowedByUserResult = crate::FutureFollowedByUserResult;
    type FutureUnfollowedByUserResult = crate::FutureUnfollowedByUserResult;
    type FutureGetUsernameResult = crate::FutureGetUsernameResult;
    type FutureGetFollowersResult = crate::FutureGetFollowersResult;
    type FutureGetFollowingResult = crate::FutureGetFollowingResult;
}
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureCreateUserResult
for FutureCreateUserResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, ()>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:user-management/user-api.{create-user}"
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
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureUpdateProfilePictureResult
for FutureUpdateProfilePictureResult {
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
                            "component:user-management/user-api.{update-profile-picture}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureGetProfilePictureResult
for FutureGetProfilePictureResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<Vec<u8>, ()>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:user-management/user-api.{get-profile-picture}"
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
                                    .list_elements(|item| item.u8().expect("u8 not found"))
                                    .expect("list not found"),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureFollowUserResult
for FutureFollowUserResult {
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
                            "component:user-management/user-api.{follow-user}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureUnfollowUserResult
for FutureUnfollowUserResult {
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
                            "component:user-management/user-api.{unfollow-user}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureFollowedByUserResult
for FutureFollowedByUserResult {
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
                            "component:user-management/user-api.{followed-by-user}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureUnfollowedByUserResult
for FutureUnfollowedByUserResult {
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
                            "component:user-management/user-api.{unfollowed-by-user}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureGetUsernameResult
for FutureGetUsernameResult {
    fn subscribe(&self) -> bindings::wasi::io::poll::Pollable {
        let pollable = self.future_invoke_result.subscribe();
        let pollable = unsafe {
            bindings::wasi::io::poll::Pollable::from_handle(pollable.take_handle())
        };
        pollable
    }
    fn get(&self) -> Option<Result<String, ()>> {
        self.future_invoke_result
            .get()
            .map(|result| {
                let result = result
                    .expect(
                        &format!(
                            "Failed to invoke remote {}",
                            "component:user-management/user-api.{get-username}"
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
                                    .string()
                                    .expect("string not found")
                                    .to_string(),
                            )
                        }
                        Err(err_value) => Err(()),
                    }
                })
            })
    }
}
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureGetFollowersResult
for FutureGetFollowersResult {
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
                            "component:user-management/user-api.{get-followers}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestFutureGetFollowingResult
for FutureGetFollowingResult {
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
                            "component:user-management/user-api.{get-following}"
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
impl crate::bindings::exports::component::user_management_stub::stub_user_management::GuestUserApi
for UserApi {
    fn new(location: crate::bindings::golem::rpc::types::Uri) -> Self {
        let location = golem_wasm_rpc::Uri {
            value: location.value,
        };
        Self {
            rpc: WasmRpc::new(&location),
        }
    }
    fn blocking_create_user(&self, username: String) -> Result<String, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{create-user}",
                &[WitValue::builder().string(&username)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{create-user}"
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
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn create_user(
        &self,
        username: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureCreateUserResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{create-user}",
                &[WitValue::builder().string(&username)],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureCreateUserResult::new(FutureCreateUserResult {
            future_invoke_result: result,
        })
    }
    fn blocking_update_profile_picture(
        &self,
        user_id: String,
        picture_data: Vec<u8>,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{update-profile-picture}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder()
                        .list_fn(
                            &picture_data,
                            |item, item_builder| { item_builder.u8(*item) },
                        ),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{update-profile-picture}"
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
    fn update_profile_picture(
        &self,
        user_id: String,
        picture_data: Vec<u8>,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUpdateProfilePictureResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{update-profile-picture}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder()
                        .list_fn(
                            &picture_data,
                            |item, item_builder| { item_builder.u8(*item) },
                        ),
                ],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUpdateProfilePictureResult::new(FutureUpdateProfilePictureResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_profile_picture(&self, user_id: String) -> Result<Vec<u8>, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{get-profile-picture}",
                &[WitValue::builder().string(&user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{get-profile-picture}"
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
                            .list_elements(|item| item.u8().expect("u8 not found"))
                            .expect("list not found"),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn get_profile_picture(
        &self,
        user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetProfilePictureResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{get-profile-picture}",
                &[WitValue::builder().string(&user_id)],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetProfilePictureResult::new(FutureGetProfilePictureResult {
            future_invoke_result: result,
        })
    }
    fn blocking_follow_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{follow-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{follow-user}"
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
    fn follow_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureFollowUserResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{follow-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureFollowUserResult::new(FutureFollowUserResult {
            future_invoke_result: result,
        })
    }
    fn blocking_unfollow_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{unfollow-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{unfollow-user}"
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
    fn unfollow_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUnfollowUserResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{unfollow-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUnfollowUserResult::new(FutureUnfollowUserResult {
            future_invoke_result: result,
        })
    }
    fn blocking_followed_by_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{followed-by-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{followed-by-user}"
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
    fn followed_by_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureFollowedByUserResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{followed-by-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureFollowedByUserResult::new(FutureFollowedByUserResult {
            future_invoke_result: result,
        })
    }
    fn blocking_unfollowed_by_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> Result<bool, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{unfollowed-by-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{unfollowed-by-user}"
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
    fn unfollowed_by_user(
        &self,
        user_id: String,
        target_user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUnfollowedByUserResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{unfollowed-by-user}",
                &[
                    WitValue::builder().string(&user_id),
                    WitValue::builder().string(&target_user_id),
                ],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureUnfollowedByUserResult::new(FutureUnfollowedByUserResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_username(&self, user_id: String) -> Result<String, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{get-username}",
                &[WitValue::builder().string(&user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{get-username}"
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
                            .string()
                            .expect("string not found")
                            .to_string(),
                    )
                }
                Err(err_value) => Err(()),
            }
        })
    }
    fn get_username(
        &self,
        user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetUsernameResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{get-username}",
                &[WitValue::builder().string(&user_id)],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetUsernameResult::new(FutureGetUsernameResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_followers(&self, user_id: String) -> Result<Vec<String>, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{get-followers}",
                &[WitValue::builder().string(&user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{get-followers}"
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
    fn get_followers(
        &self,
        user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetFollowersResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{get-followers}",
                &[WitValue::builder().string(&user_id)],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetFollowersResult::new(FutureGetFollowersResult {
            future_invoke_result: result,
        })
    }
    fn blocking_get_following(&self, user_id: String) -> Result<Vec<String>, ()> {
        let result = self
            .rpc
            .invoke_and_await(
                "component:user-management/user-api.{get-following}",
                &[WitValue::builder().string(&user_id)],
            )
            .expect(
                &format!(
                    "Failed to invoke-and-await remote {}",
                    "component:user-management/user-api.{get-following}"
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
    fn get_following(
        &self,
        user_id: String,
    ) -> crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetFollowingResult {
        let result = self
            .rpc
            .async_invoke_and_await(
                "component:user-management/user-api.{get-following}",
                &[WitValue::builder().string(&user_id)],
            );
        crate::bindings::exports::component::user_management_stub::stub_user_management::FutureGetFollowingResult::new(FutureGetFollowingResult {
            future_invoke_result: result,
        })
    }
}
bindings::export!(Component with_types_in bindings);
