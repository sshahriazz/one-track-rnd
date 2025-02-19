use sea_orm::prelude::Uuid;
use sea_orm::{DatabaseConnection, DeleteResult};

use crate::dtos::task_dto::{TaskCreateDto, TaskUpdateDto};
use crate::entities::{prelude::*, task};

use crate::repository::task_repository::{TaskMutation as TM, TaskQuery as TQ};
use crate::utils::error::AppError;

pub struct TaskService;

impl TaskService {
    pub async fn get_tasks(db: &DatabaseConnection) -> Result<Vec<task::Model>, AppError> {
        TQ::get_tasks(db).await
    }

    pub async fn get_task_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<task::Model>, AppError> {
        TQ::get_task_by_id(db, id).await
    }

    pub async fn get_tasks_by_section_id(
        db: &DatabaseConnection,
        section_id: Uuid,
    ) -> Result<Vec<task::Model>, AppError> {
        TQ::get_tasks_by_section_id(db, section_id).await
    }

    pub async fn create_task(
        db: &DatabaseConnection,
        data: TaskCreateDto,
        section_id: Uuid,
    ) -> Result<task::Model, AppError> {
        TM::create_task(db, data, section_id).await
    }

    pub async fn update_task(
        db: &DatabaseConnection,
        id: Uuid,
        data: TaskUpdateDto,
    ) -> Result<task::Model, AppError> {
        TM::update_task(db, id, data).await
    }

    pub async fn delete_task(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
        TM::delete_task(db, id).await
    }
}
