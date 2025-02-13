use std::sync::Arc;

use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

use crate::{
    dtos::project_dto::{ProjectCreateDto, ProjectUpdateDto},
    entities::{prelude::*, *},
    utils::error::AppError,
};

#[derive(Debug)]
pub struct ProjectQuery;

pub struct ProjectMutation;

impl ProjectMutation {
    pub async fn create_project(
        db: &DatabaseConnection,
        data: ProjectCreateDto,
    ) -> Result<project::Model, AppError> {
        let project = project::ActiveModel {
            name: ActiveValue::set(data.name),
            ..Default::default()
        };
        Project::insert(project)
            .exec_with_returning(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create project: {}", e)))
    }
    pub async fn update_project(
        db: &DatabaseConnection,
        id: Uuid,
        data: ProjectUpdateDto,
    ) -> Result<project::Model, AppError> {
        use sea_orm::{ColumnTrait, QueryFilter};

        // Create the active model with the new version
        let project = project::ActiveModel {
            id: ActiveValue::set(id),
            name: ActiveValue::set(data.name),
            version: ActiveValue::set(data.version + 1),
            ..Default::default()
        };

        // Update with version check
        let result = Project::update_many()
            .set(project)
            .filter(project::Column::Id.eq(id))
            .filter(project::Column::Version.eq(data.version))
            .exec(db)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Couldn't update project: {:?}", e))
            })?;

        if result.rows_affected == 0 {
            return Err(AppError::Conflict(
                "Project was modified by another user. Please refresh and try again.".to_string(),
            ));
        }

        // Fetch and return the updated project
        Project::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to fetch updated project: {:?}", e))
            })?
            .ok_or_else(|| AppError::NotFound(format!("Project with id {} not found", id)))
    }

    pub async fn delete_project(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        Project::delete_by_id(id)
            .exec(db)
            .await
            .map(|res| res.rows_affected)
            .map_err(|e| AppError::InternalServerError(format!("Failed to delete project: {}", e)))
    }
}

impl ProjectQuery {
    pub async fn get_projects(db: &DatabaseConnection) -> Result<Vec<project::Model>, AppError> {
        Project::find()
            .all(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch projects: {}", e)))
    }

    pub async fn get_project_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<project::Model>, AppError> {
        Project::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch project: {}", e)))
    }
}
