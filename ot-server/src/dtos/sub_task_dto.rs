use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTaskDto {
    pub name: String,
    pub section_id: Uuid,
    pub id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubTaskCreateDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub section_id: Uuid,
    pub task_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubTaskUpdateDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub section_id: Uuid,
    pub task_id: Option<Uuid>,
}
