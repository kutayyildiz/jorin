use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantType {
    User,
    Interceptor,
    Orchestrator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "type")]
    pub r#type: ParticipantType,
    pub id: String,
}
