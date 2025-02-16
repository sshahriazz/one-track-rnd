use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr,
    DeleteResult, EntityTrait, QueryFilter,
};

use crate::{
    dtos::sub_task_dto::{SubTaskCreateDto, SubTaskUpdateDto},
    entities::{prelude::*, *},
    utils::error::AppError,
};

pub struct SubTaskMutation;
pub struct SubTaskQuery;

impl SubTaskMutation {
    pub async fn create_sub_task_for_task_and_section(
        db: &DatabaseConnection,
        data: SubTaskCreateDto,
        section_id: Uuid,
        task_id: Option<Uuid>,
    ) -> Result<sub_task::Model, AppError> {
        let sub_task = sub_task::ActiveModel {
            task_id: ActiveValue::set(task_id),
            section_id: ActiveValue::set(section_id),
            name: ActiveValue::set(data.name),
            ..Default::default()
        };

        SubTask::insert(sub_task)
            .exec_with_returning(db)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to create sub task: {:?}", e))
            })
    }

    pub async fn update_sub_task(
        db: &DatabaseConnection,
        id: Uuid,
        data: SubTaskUpdateDto,
    ) -> Result<sub_task::Model, AppError> {
        let sub_task = sub_task::ActiveModel {
            id: ActiveValue::set(id),
            name: ActiveValue::set(data.name),
            ..Default::default()
        };
        sub_task.update(db).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to update sub task: {:?}", e))
        })
    }

    pub async fn delete_sub_task(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<DeleteResult, AppError> {
        let sub_task = sub_task::ActiveModel {
            id: ActiveValue::set(id),
            ..Default::default()
        };
        sub_task.delete(db).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to delete sub task: {:?}", e))
        })
    }
}

impl SubTaskQuery {
    pub async fn get_sub_task_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<sub_task::Model>, AppError> {
        let result = SubTask::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to get sub task: {:?}", e)));
        result
    }

    pub async fn get_sub_tasks(db: &DatabaseConnection) -> Result<Vec<sub_task::Model>, AppError> {
        let result = SubTask::find().all(db).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to get sub tasks: {:?}", e))
        });
        result
    }

    pub async fn get_sub_tasks_by_section_id(
        db: &DatabaseConnection,
        section_id: Uuid,
    ) -> Result<Vec<sub_task::Model>, AppError> {
        let result = SubTask::find()
            .filter(sub_task::Column::SectionId.eq(section_id))
            .all(db)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get sub tasks: {:?}", e))
            });
        result
    }

    pub async fn get_sub_tasks_by_task_id(
        db: &DatabaseConnection,
        task_id: Uuid,
    ) -> Result<Vec<sub_task::Model>, AppError> {
        let result = SubTask::find()
            .filter(sub_task::Column::TaskId.eq(task_id))
            .all(db)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get sub tasks: {:?}", e))
            });
        result
    }
}
