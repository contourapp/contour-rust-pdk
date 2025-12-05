use chrono::{DateTime, Utc};
use extism_pdk::{Json, ToBytes};
use serde::{Deserialize, Serialize};

use crate::inputs::{EntryInput, ResourceInput, TagInput};

#[derive(Debug, Clone, Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
pub struct ExtractResponse {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToBytes)]
#[encoding(Json)]
pub enum TransformResponse<I> {
    EntryInput(EntryInput),
    TagInput(TagInput<I>),
    ResourceInput(ResourceInput<I>),
    None,
}
