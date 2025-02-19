use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDto {
    pub name: String,
    pub section_id: Uuid,
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TaskCreateDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub section_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TaskUpdateDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub section_id: Uuid,
}
