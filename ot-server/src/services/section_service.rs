use sea_orm::{prelude::Uuid, DatabaseConnection};

use crate::{
    dtos::section_dto::{SectionCreateDto, SectionUpdateDto},
    entities::section,
    repository::section_repository::{SectionMutation as SM, SectionQuery as SQ},
    utils::error::AppError,
};

pub struct SectionService;

impl SectionService {
    pub async fn get_sections(db: &DatabaseConnection) -> Result<Vec<section::Model>, AppError> {
        SQ::get_sections(db).await
    }

    pub async fn get_section_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<section::Model>, AppError> {
        SQ::get_section_by_id(db, id).await
    }

    pub async fn create_section(
        db: &DatabaseConnection,
        data: SectionCreateDto,
    ) -> Result<section::Model, AppError> {
        SM::create_section(db, data).await
    }

    pub async fn update_section(
        db: &DatabaseConnection,
        id: Uuid,
        data: SectionUpdateDto,
    ) -> Result<section::Model, AppError> {
        SM::update_section(db, id, data).await
    }

    pub async fn delete_section(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        SM::delete_section(db, id).await
    }
}
