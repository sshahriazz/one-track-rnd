use sea_orm::prelude::Uuid;
use sea_orm::{DatabaseConnection, DeleteResult};

use crate::dtos::sub_task_dto::{SubTaskCreateDto, SubTaskUpdateDto};
use crate::entities::{prelude::*, sub_task};

use crate::repository::sub_task_repository::{SubTaskMutation as STM, SubTaskQuery as STQ};
use crate::utils::error::AppError;

pub struct SubTaskService;

impl SubTaskService {
    pub async fn get_sub_tasks(db: &DatabaseConnection) -> Result<Vec<sub_task::Model>, AppError> {
        STQ::get_sub_tasks(db).await
    }

    pub async fn get_sub_task_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<sub_task::Model>, AppError> {
        STQ::get_sub_task_by_id(db, id).await
    }

    pub async fn get_sub_tasks_by_section_id(
        db: &DatabaseConnection,
        section_id: Uuid,
    ) -> Result<Vec<sub_task::Model>, AppError> {
        STQ::get_sub_tasks_by_section_id(db, section_id).await
    }

    pub async fn get_sub_tasks_by_task_id(
        db: &DatabaseConnection,
        task_id: Uuid,
    ) -> Result<Vec<sub_task::Model>, AppError> {
        STQ::get_sub_tasks_by_task_id(db, task_id).await
    }

    pub async fn create_sub_task_for_task_and_section(
        db: &DatabaseConnection,
        data: SubTaskCreateDto,
        section_id: Uuid,
        task_id: Option<Uuid>,
    ) -> Result<sub_task::Model, AppError> {
        STM::create_sub_task_for_task_and_section(db, data, section_id, task_id).await
    }

    pub async fn update_sub_task(
        db: &DatabaseConnection,
        id: Uuid,
        data: SubTaskUpdateDto,
    ) -> Result<sub_task::Model, AppError> {
        STM::update_sub_task(db, id, data).await
    }

    pub async fn delete_sub_task(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<DeleteResult, AppError> {
        STM::delete_sub_task(db, id).await
    }
}
