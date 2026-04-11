use actrpc_core::json_rpc::JsonRpcMessage;
use actrpc_transport::TransportTarget;

pub trait Orchestrator {
    type Error;

    fn handle(
        &self,
        message: JsonRpcMessage,
        destination: TransportTarget,
    ) -> Result<JsonRpcMessage, Self::Error>;
}
