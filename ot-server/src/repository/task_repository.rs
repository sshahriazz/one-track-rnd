use crate::{
    dtos::task_dto::{TaskCreateDto, TaskUpdateDto},
    entities::{prelude::*, *},
    utils::error::AppError,
};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr,
    DeleteResult, EntityTrait, QueryFilter,
};

pub struct TaskMutation;

pub struct TaskQuery;

impl TaskMutation {
    pub async fn create_task(
        db: &DatabaseConnection,
        data: TaskCreateDto,
        section_id: Uuid,
    ) -> Result<task::Model, AppError> {
        let task = task::ActiveModel {
            name: ActiveValue::set(data.name),
            section_id: ActiveValue::set(section_id),
            ..Default::default()
        };
        Task::insert(task)
            .exec_with_returning(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create task: {}", e)))
    }

    pub async fn update_task(
        db: &DatabaseConnection,
        id: Uuid,
        data: TaskUpdateDto,
    ) -> Result<task::Model, AppError> {
        let task = task::ActiveModel {
            id: ActiveValue::set(id),
            name: ActiveValue::set(data.name),
            section_id: ActiveValue::set(data.section_id),
            ..Default::default()
        };
        task.update(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to update task: {}", e)))
    }

    pub async fn delete_task(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
        let task = task::ActiveModel {
            id: ActiveValue::set(id),
            ..Default::default()
        };
        task.delete(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to delete task: {}", e)))
    }
}

impl TaskQuery {
    pub async fn get_tasks(db: &DatabaseConnection) -> Result<Vec<task::Model>, AppError> {
        Task::find()
            .all(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get tasks: {}", e)))
    }

    pub async fn get_task_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<task::Model>, AppError> {
        Task::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get task: {}", e)))
    }
    pub async fn get_tasks_by_section_id(
        db: &DatabaseConnection,
        section_id: Uuid,
    ) -> Result<Vec<task::Model>, AppError> {
        Task::find()
            .filter(task::Column::SectionId.eq(section_id))
            .all(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get tasks: {}", e)))
    }
}
