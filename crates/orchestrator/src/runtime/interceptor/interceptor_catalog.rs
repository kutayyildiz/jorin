use crate::{
    error::ActionExecutionError,
    interceptor::Interceptor,
    runtime::interceptor::{
        ImmutableInterceptorPipeline, InterceptorPolicy, WorkingInterceptorPipeline,
    },
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct InterceptorCatalogEntry {
    pub name: String,
    pub policy: InterceptorPolicy,
    pub interceptor: Arc<dyn Interceptor>,
}

impl core::fmt::Debug for InterceptorCatalogEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InterceptorCatalogEntry")
            .field("name", &self.name)
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct InterceptorCatalog {
    entries: HashMap<String, InterceptorCatalogEntry>,
    outbound_pipeline: ImmutableInterceptorPipeline,
    inbound_pipeline: ImmutableInterceptorPipeline,
}

impl InterceptorCatalog {
    pub fn new(
        entries: HashMap<String, InterceptorCatalogEntry>,
        outbound_pipeline: ImmutableInterceptorPipeline,
        inbound_pipeline: ImmutableInterceptorPipeline,
    ) -> Self {
        Self {
            entries,
            outbound_pipeline,
            inbound_pipeline,
        }
    }

    pub fn get_entry(&self, name: &str) -> Result<InterceptorCatalogEntry, ActionExecutionError> {
        let Some(entry) = self.entries.get(name) else {
            return Err(ActionExecutionError::NotFound {
                target: name.to_owned(),
            });
        };

        Ok(entry.clone())
    }

    pub fn entries(&self) -> Vec<InterceptorCatalogEntry> {
        self.entries.values().cloned().collect()
    }

    pub fn entries_for_names(
        &self,
        names: &[String],
    ) -> Result<Vec<InterceptorCatalogEntry>, ActionExecutionError> {
        let mut result = Vec::with_capacity(names.len());

        for name in names {
            let Some(entry) = self.entries.get(name) else {
                return Err(ActionExecutionError::NotFound {
                    target: name.clone(),
                });
            };

            result.push(entry.clone());
        }

        Ok(result)
    }

    pub fn outbound_pipeline_snapshot(&self) -> WorkingInterceptorPipeline {
        self.outbound_pipeline.snapshot()
    }

    pub fn inbound_pipeline_snapshot(&self) -> WorkingInterceptorPipeline {
        self.inbound_pipeline.snapshot()
    }
}
