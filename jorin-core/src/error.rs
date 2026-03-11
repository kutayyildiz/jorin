use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JorinError {
    #[error("action execution failed: {0}")]
    ActionExecution(#[from] ActionExecutionError),

    #[error("interceptor error: {0}")]
    Interceptor(#[from] InterceptorError),

    #[error("invalid JSON-RPC message: {0}")]
    InvalidMessage(String),

    #[error("serialization error: {message}")]
    Serialization { message: String },

    #[error("internal orchestrator error: {0}")]
    Internal(String),
}

#[derive(Debug, Error, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionExecutionError {
    #[error("invalid parameters for action '{action}'")]
    InvalidParams { action: String },

    #[error("action '{action}' forbidden by policy")]
    ForbiddenCapability { action: String },

    #[error("unsupported action '{action}'")]
    UnsupportedAction { action: String },

    #[error("target '{target}' not found")]
    TargetNotFound { target: String },

    #[error("external method error: {0}")]
    ExternalMethodError(String),

    #[error("internal orchestrator error")]
    Internal,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum InterceptorError {
    #[error("interceptor '{name}' failed: {reason}")]
    ExecutionFailed { name: String, reason: String },
    #[error("malformed response from interceptor")]
    MalformedResponse,
}
