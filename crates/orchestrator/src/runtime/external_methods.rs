use crate::error::ActionExecutionError;
use actrpc_core::{
    interception::InterceptionRequest,
    json_rpc::{
        JsonRpcId, JsonRpcMessage, JsonRpcParams, JsonRpcRequest, JsonRpcResponse,
        JsonRpcSingleMessage, JsonRpcVersion,
    },
};
use actrpc_transport::{JsonRpcClient, JsonRpcClientProvider, TransportError, TransportTarget};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalMethodEntry {
    pub name: String,
    pub target: TransportTarget,
    pub remote_method: String,
}

#[derive(Debug, Default)]
pub struct ExternalMethodCatalog {
    entries: RwLock<HashMap<String, ExternalMethodEntry>>,
}

impl ExternalMethodCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, entry: ExternalMethodEntry) -> Result<(), ActionExecutionError> {
        let mut entries = self
            .entries
            .write()
            .expect("poisoned external methods lock");

        if entries.contains_key(&entry.name) {
            return Err(ActionExecutionError::Internal {
                message: format!("duplicate external method registration: {}", entry.name),
            });
        }

        entries.insert(entry.name.clone(), entry);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<ExternalMethodEntry, ActionExecutionError> {
        let entries = self.entries.read().expect("poisoned external methods lock");
        let Some(entry) = entries.get(name) else {
            return Err(ActionExecutionError::NotFound {
                target: name.to_owned(),
            });
        };

        Ok(entry.clone())
    }

    pub fn list(&self) -> Vec<String> {
        let entries = self.entries.read().expect("poisoned external methods lock");
        let mut names: Vec<_> = entries.keys().cloned().collect();
        names.sort();
        names
    }
}

#[derive(Debug)]
pub struct TransportExternalMethodCaller<P>
where
    P: JsonRpcClientProvider<Error = TransportError>,
{
    catalog: Arc<ExternalMethodCatalog>,
    client_provider: Arc<P>,
    next_id: AtomicU64,
}

impl<P> TransportExternalMethodCaller<P>
where
    P: JsonRpcClientProvider<Error = TransportError>,
{
    pub fn new(catalog: Arc<ExternalMethodCatalog>, client_provider: Arc<P>) -> Self {
        Self {
            catalog,
            client_provider,
            next_id: AtomicU64::new(1),
        }
    }

    pub fn list_methods(&self) -> Vec<String> {
        self.catalog.list()
    }

    pub fn call(
        &self,
        _request: &InterceptionRequest,
        method: &str,
        params: Option<JsonRpcParams>,
    ) -> Result<JsonRpcResponse, ActionExecutionError> {
        let entry = self.catalog.get(method)?;
        let client = self
            .client_provider
            .get_client(&entry.target)
            .map_err(map_transport_error)?;

        let rpc_request = JsonRpcMessage::Single(JsonRpcSingleMessage::Request(JsonRpcRequest {
            jsonrpc: JsonRpcVersion::V2_0,
            id: JsonRpcId::Number(self.next_id.fetch_add(1, Ordering::Relaxed).into()),
            method: entry.remote_method,
            params,
        }));

        let response = client.send(rpc_request).map_err(map_transport_error)?;

        match response {
            JsonRpcMessage::Single(JsonRpcSingleMessage::Response(resp)) => Ok(resp),
            other => Err(ActionExecutionError::DependencyFailed {
                dependency: method.to_owned(),
                message: format!("unexpected external method response shape: {other:?}"),
            }),
        }
    }
}

fn map_transport_error(error: TransportError) -> ActionExecutionError {
    ActionExecutionError::DependencyFailed {
        dependency: "external".to_owned(),
        message: error.to_string(),
    }
}
