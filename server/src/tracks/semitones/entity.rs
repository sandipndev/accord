use crate::primitives::{SemitoneId, SemitoneStatus, TrackId};
use chrono::{DateTime, Utc};

pub struct Semitone {
    pub id: SemitoneId,
    pub track_id: TrackId,
    pub shift: i32,
    pub status: SemitoneStatus,
    pub created_at: DateTime<Utc>,
}

pub struct NewSemitone {
    pub track_id: TrackId,
    pub shift: i32,
}
