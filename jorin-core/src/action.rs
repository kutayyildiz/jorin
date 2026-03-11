use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "action_params", rename_all = "snake_case")]
pub enum Action {
    /// Replaces or rewrites params of the currently inspected message.
    /// Spec says: "<any>"
    ModifyParams(Value),

    /// Calls an orchestrator-known external method.
    CallExternalMethod {
        method_name: String,
        method_params: Value,
    },

    /// Returns available external methods.
    ListExternalMethods,

    /// Returns current interceptor order.
    GetInterceptorOrder,

    /// Returns current interceptor enabled/executed state.
    GetInterceptorState,

    /// Returns current interceptor policy.
    GetInterceptorPolicy,

    /// Enables the named interceptors.
    EnableInterceptors(Vec<String>),

    /// Disables the named interceptors.
    DisableInterceptors(Vec<String>),

    /// Returns transcript records.
    GetTranscript,
}
