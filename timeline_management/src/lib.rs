#[allow(warnings)]
mod bindings;

use bindings::exports::component::timeline_management::api::*;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
