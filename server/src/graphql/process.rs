use async_graphql::*;

use crate::primitives::{ProcessId, ProcessStatus};

#[derive(SimpleObject)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub youtube_url: String,
    pub status: ProcessStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::process::Process> for Process {
    fn from(process: crate::process::Process) -> Self {
        Self {
            id: process.id,
            name: process.name,
            youtube_url: process.youtube_url.to_string(),
            status: process.status,
            created_at: process.created_at,
        }
    }
}
