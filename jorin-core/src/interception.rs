use crate::{action::Action, executed_action::ExecutedAction, json_rpc::JsonRpcMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptionRequest {
    pub origin: String,
    pub message: JsonRpcMessage, // ← see below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_actions: Option<Vec<ExecutedAction>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptionDecision {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "final")]
    pub r#final: bool,
}
