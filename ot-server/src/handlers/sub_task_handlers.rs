use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;

use crate::{
    dtos::sub_task_dto::{SubTaskCreateDto, SubTaskUpdateDto},
    AppState,
};
use crate::{
    dtos::{sub_task_dto::SubTaskDto, task_dto::TaskDto},
    utils::error::AppError,
};
use crate::{entities::sub_task, services::sub_task_service::SubTaskService};

#[derive(Debug, serde::Deserialize)]
pub struct CreateSubTaskPath {
    section_id: Uuid,
    task_id: Option<Uuid>,
}

pub struct SubTaskHandlers;

impl SubTaskHandlers {
    pub async fn get_sub_tasks_handler(State(state): State<AppState>) -> impl IntoResponse {
        match SubTaskService::get_sub_tasks(&state.db).await {
            Ok(tasks) => {
                let result_data = tasks
                    .iter()
                    .map(|t| SubTaskDto {
                        id: t.id,
                        name: t.name.clone(),
                        section_id: t.section_id,
                        created_at: t.created_at,
                        updated_at: t.updated_at,
                    })
                    .collect::<Vec<SubTaskDto>>();
                (StatusCode::OK, Json(result_data)).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn get_sub_task_by_id_handler(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match SubTaskService::get_sub_task_by_id(&state.db, id).await {
            Ok(task) => match task {
                Some(task) => (
                    StatusCode::OK,
                    Json(SubTaskDto {
                        id: task.id,
                        name: task.name.clone(),
                        section_id: task.section_id,
                        created_at: task.created_at,
                        updated_at: task.updated_at,
                    }),
                )
                    .into_response(),
                None => (StatusCode::NOT_FOUND, "SubTask not found").into_response(),
            },
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn get_sub_tasks_by_section_id_handler(
        State(state): State<AppState>,
        Path(section_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match SubTaskService::get_sub_tasks_by_section_id(&state.db, section_id).await {
            Ok(tasks) => {
                let result_data = tasks
                    .iter()
                    .map(|t| SubTaskDto {
                        id: t.id,
                        name: t.name.clone(),
                        section_id: t.section_id,
                        created_at: t.created_at,
                        updated_at: t.updated_at,
                    })
                    .collect::<Vec<SubTaskDto>>();
                (StatusCode::OK, Json(result_data)).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn get_sub_tasks_by_task_id_handler(
        Path(task_id): Path<Uuid>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        match SubTaskService::get_sub_tasks_by_task_id(&state.db, task_id).await {
            Ok(tasks) => {
                let result_data = tasks
                    .iter()
                    .map(|t| SubTaskDto {
                        id: t.id,
                        name: t.name.clone(),
                        section_id: t.section_id,
                        created_at: t.created_at,
                        updated_at: t.updated_at,
                    })
                    .collect::<Vec<SubTaskDto>>();
                (StatusCode::OK, Json(result_data)).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn create_sub_task_handler(
        State(state): State<AppState>,
        Path(path_params): Path<CreateSubTaskPath>,
        Json(data): Json<SubTaskCreateDto>,
    ) -> impl IntoResponse {
        match SubTaskService::create_sub_task_for_task_and_section(
            &state.db,
            data,
            path_params.section_id,
            path_params.task_id,
        )
        .await
        {
            Ok(sub_task) => (
                StatusCode::CREATED,
                Json(SubTaskDto {
                    id: sub_task.id,
                    name: sub_task.name.clone(),
                    section_id: sub_task.section_id,
                    created_at: sub_task.created_at,
                    updated_at: sub_task.updated_at,
                }),
            )
                .into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn update_sub_task_handler(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        Json(data): Json<SubTaskUpdateDto>,
    ) -> impl IntoResponse {
        match SubTaskService::update_sub_task(&state.db, id, data).await {
            Ok(task) => (
                StatusCode::OK,
                Json(SubTaskDto {
                    id: task.id,
                    name: task.name.clone(),
                    section_id: task.section_id,
                    created_at: task.created_at,
                    updated_at: task.updated_at,
                }),
            )
                .into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn delete_sub_task_handler(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match SubTaskService::delete_sub_task(&state.db, id).await {
            Ok(_) => (StatusCode::OK, Json("SubTask deleted")).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }
}
