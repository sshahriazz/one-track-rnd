use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

pub struct TaskDto {
    pub name: String,
    pub section_id: Uuid,
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize)]
pub struct TaskCreateDto {
    pub name: String,
    pub section_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TaskUpdateDto {
    pub name: String,
    pub section_id: Uuid,
}
