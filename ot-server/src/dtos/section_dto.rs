use chrono::{DateTime, FixedOffset};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize)]
pub struct SectionDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Validate, Deserialize)]
pub struct SectionCreateDto {
    pub name: String,
    pub project_id: Uuid,
}

#[derive(Validate, Deserialize)]
pub struct SectionUpdateDto {
    pub name: String,
}
