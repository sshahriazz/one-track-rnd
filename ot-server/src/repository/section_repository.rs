use futures::future::Select;
use sea_orm::{
    prelude::{DateTimeWithTimeZone, Uuid},
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    QueryFilter, Related, RelationTrait,
};
use sea_orm_migration::seaql_migrations::Column;

use crate::{
    dtos::section_dto::{SectionCreateDto, SectionUpdateDto},
    entities::{prelude::*, *},
    utils::error::AppError,
};

pub struct SectionQuery;
pub struct SectionMutation;

impl SectionMutation {
    pub async fn create_section(
        db: &DatabaseConnection,
        data: SectionCreateDto,
    ) -> Result<section::Model, AppError> {
        let section = section::ActiveModel {
            name: ActiveValue::set(data.name),
            project_id: ActiveValue::set(data.project_id),
            ..Default::default()
        };
        Section::insert(section)
            .exec_with_returning(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create section: {}", e)))
    }

    pub async fn update_section(
        db: &DatabaseConnection,
        id: Uuid,
        data: SectionUpdateDto,
    ) -> Result<section::Model, AppError> {
        let section = section::ActiveModel {
            id: ActiveValue::set(id),
            name: ActiveValue::set(data.name),
            ..Default::default()
        };
        section.update(db).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to update section: {:?}", e))
        })
    }

    pub async fn delete_section(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        Section::delete_by_id(id)
            .exec(db)
            .await
            .map(|res| res.rows_affected)
            .map_err(|e| AppError::InternalServerError(format!("Failed to delete section: {}", e)))
    }
}

impl SectionQuery {
    pub async fn get_sections(db: &DatabaseConnection) -> Result<Vec<section::Model>, AppError> {
        Section::find()
            .all(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get sections: {}", e)))
    }

    pub async fn get_section_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<section::Model>, AppError> {
        Section::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get section: {}", e)))
    }
    pub async fn get_sections_by_project_id(
        db: &DatabaseConnection,
        project_id: Uuid,
    ) -> Result<Vec<section::Model>, AppError> {
        Section::find()
            .filter(section::Column::ProjectId.eq(project_id))
            .all(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get sections: {}", e)))
    }
}
