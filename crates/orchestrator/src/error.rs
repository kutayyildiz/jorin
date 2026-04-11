#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error("unknown action kind: {kind}")]
    UnknownAction {
        kind: actrpc_core::action::ActionKind,
    },

    #[error("interceptor registry error: {message}")]
    InterceptorRegistry { message: String },

    #[error("action registry error: {message}")]
    ActionRegistry { message: String },

    #[error("interceptor invocation failed for {name}: {message}")]
    InterceptorInvocation { name: String, message: String },

    #[error("downstream forwarding failed: {message}")]
    Forwarding { message: String },

    #[error(transparent)]
    Transport(#[from] actrpc_transport::TransportError),

    #[error(transparent)]
    ActionCodec(#[from] actrpc_core::error::ActionCodecError),

    #[error(transparent)]
    Interception(#[from] actrpc_core::error::InterceptionError),

    #[error(transparent)]
    Protocol(#[from] actrpc_core::error::ProtocolError),

    #[error(transparent)]
    Codec(#[from] actrpc_core::error::CodecError),

    #[error("internal orchestrator error: {message}")]
    Internal { message: String },
}
