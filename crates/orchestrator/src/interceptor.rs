use actrpc_core::{
    interception::InterceptionPhase,
    interceptor::{InterceptorCapabilities, InterceptorPolicy},
};
use actrpc_transport::TransportTarget;

pub trait InterceptorRegistry {
    type Error;

    fn interceptors_for_phase(
        &self,
        phase: InterceptionPhase,
    ) -> Result<Vec<InterceptorEntry>, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct InterceptorEntry {
    pub name: String,
    pub target: TransportTarget,
    pub capabilities: InterceptorCapabilities,
    pub policy: Option<InterceptorPolicy>,
    pub priority: i32,
    pub enabled: bool,
}
