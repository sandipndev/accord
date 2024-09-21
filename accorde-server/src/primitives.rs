use serde::Serialize;

use crate::entity_id;

entity_id!(ProcessId);
async_graphql::scalar!(ProcessId);

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "process_status", rename_all = "UPPERCASE")]
pub enum ProcessStatus {
    Pending,
    Downloading,
    Converting,
    Done,
}
