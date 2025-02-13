use crate::{
    dtos::task_dto::{TaskCreateDto, TaskUpdateDto},
    entities::{prelude::*, *},
    utils::error::AppError,
};

use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, DeleteResult,
    EntityTrait,
};

pub async fn create_task(
    db: &DatabaseConnection,
    data: TaskCreateDto,
    section_id: Uuid,
) -> Result<task::Model, DbErr> {
    let task = task::ActiveModel {
        name: ActiveValue::set(data.name),
        section_id: ActiveValue::set(section_id),
        ..Default::default()
    };
    let result = Task::insert(task).exec_with_returning(db).await?;
    Ok(result)
}

pub async fn get_tasks(db: &DatabaseConnection) -> Result<Vec<task::Model>, AppError> {
    let tasks = Task::find()
        .all(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to get tasks: {}", e)))?;
    Ok(tasks)
}

pub async fn get_task_by_id(db: &DatabaseConnection, id: Uuid) -> Result<task::Model, AppError> {
    let task = Task::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to get task: {}", e)))?;
    Ok(task.unwrap())
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
    let result = task
        .update(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to update task: {}", e)))?;
    Ok(result)
}

pub async fn delete_task(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
    let task = task::ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    };
    let result = task
        .delete(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to delete task: {}", e)))?;
    Ok(result)
}
