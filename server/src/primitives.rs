use serde::{Deserialize, Serialize};

use crate::entity_id;

entity_id!(ProcessId);
async_graphql::scalar!(ProcessId);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, async_graphql::Enum,
)]
#[sqlx(type_name = "process_status", rename_all = "UPPERCASE")]
pub enum ProcessStatus {
    Pending,
    Downloading,
    Downloaded,
    Converting,
    Converted,
    Done,
}
