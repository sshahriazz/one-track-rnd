use chrono::{DateTime, FixedOffset};

use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ProjectCreateDto {
    #[validate(length(
        min = 1,
        max = 5,
        message = "Project name must be between 1 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProjectUpdateDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Project name must be between 1 and 100 characters"
    ))]
    pub name: String,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub version: i32,
}
