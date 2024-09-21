use crate::primitives::{ProcessId, ProcessStatus};

#[derive(Debug)]
pub struct Process {
    pub id: ProcessId,
    pub youtube_url: url::Url,
    pub status: ProcessStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewProcess {
    pub youtube_url: url::Url,
}
