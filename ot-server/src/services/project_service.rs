use sea_orm::{prelude::Uuid, DatabaseConnection};

use crate::{
    dtos::project_dto::{ProjectCreateDto, ProjectUpdateDto},
    entities::project,
    repository::{project_repository::ProjectMutation, project_repository::ProjectQuery},
    utils::error::AppError,
};

#[derive(Debug)]
pub struct ProjectService;

impl ProjectService {
    pub async fn create_project(
        db: &DatabaseConnection,
        data: ProjectCreateDto,
    ) -> Result<project::Model, AppError> {
        // Add any business logic validation here before creating
        if data.name.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Project name cannot be empty".to_string(),
            ));
        }

        ProjectMutation::create_project(db, data).await
    }

    pub async fn update_project(
        db: &DatabaseConnection,
        id: Uuid,
        data: ProjectUpdateDto,
    ) -> Result<project::Model, AppError> {
        // Validate if project exists
        if ProjectQuery::get_project_by_id(db, id).await?.is_none() {
            return Err(AppError::NotFound(format!(
                "Project with id {} not found",
                id
            )));
        }

        ProjectMutation::update_project(db, id, data).await
    }

    pub async fn delete_project(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        // Validate if project exists
        if ProjectQuery::get_project_by_id(db, id).await?.is_none() {
            return Err(AppError::NotFound(format!(
                "Project with id {} not found",
                id
            )));
        }

        // Add any cleanup logic here if needed (e.g., deleting related records)
        ProjectMutation::delete_project(db, id).await
    }

    pub async fn get_projects(db: &DatabaseConnection) -> Result<Vec<project::Model>, AppError> {
        ProjectQuery::get_projects(db).await
    }

    pub async fn get_project_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<project::Model>, AppError> {
        ProjectQuery::get_project_by_id(db, id).await
    }
}
