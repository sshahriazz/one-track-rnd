use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_valid::Valid;
use reqwest::StatusCode;
use sea_orm::prelude::Uuid;

use crate::{
    dtos::task_dto::{TaskCreateDto, TaskDto, TaskUpdateDto},
    services::task_service::TaskService,
    utils::error::AppError,
    AppState,
};

pub struct TaskMutationHandlers;
pub struct TaskQueryHandlers;

impl TaskMutationHandlers {
    pub async fn create_task_handler(
        state: State<AppState>,
        Path(section_id): Path<Uuid>,
        Valid(Json(payload)): Valid<Json<TaskCreateDto>>,
    ) -> impl IntoResponse {
        match TaskService::create_task(&state.db, payload, section_id).await {
            Ok(task) => (
                StatusCode::OK,
                Json(TaskDto {
                    id: task.id,
                    name: task.name,
                    section_id: task.section_id,
                    created_at: task.created_at,
                    updated_at: task.updated_at,
                }),
            )
                .into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn update_task_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
        Valid(Json(payload)): Valid<Json<TaskUpdateDto>>,
    ) -> impl IntoResponse {
        match TaskService::update_task(&state.db, id, payload).await {
            Ok(task) => (
                StatusCode::OK,
                Json(TaskDto {
                    id: task.id,
                    name: task.name,
                    section_id: task.section_id,
                    created_at: task.created_at,
                    updated_at: task.updated_at,
                }),
            )
                .into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn delete_task_handler(
        state: State<AppState>,
        Path(task_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match TaskService::delete_task(&state.db, task_id).await {
            Ok(task) => {
                (StatusCode::OK, format!("Task with id {} deleted", task_id)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }
}

impl TaskQueryHandlers {
    pub async fn get_tasks_handler(state: State<AppState>) -> impl IntoResponse {
        match TaskService::get_tasks(&state.db).await {
            Ok(tasks) => {
                let result_data = tasks
                    .iter()
                    .map(|t| TaskDto {
                        id: t.id,
                        name: t.name.clone(),
                        section_id: t.section_id,
                        created_at: t.created_at,
                        updated_at: t.updated_at,
                    })
                    .collect::<Vec<TaskDto>>();
                (StatusCode::OK, Json(result_data)).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }

    pub async fn get_task_by_id_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match TaskService::get_task_by_id(&state.db, id).await {
            Ok(Some(task)) => (
                StatusCode::OK,
                Json(TaskDto {
                    id: task.id,
                    name: task.name,
                    section_id: task.section_id,
                    created_at: task.created_at,
                    updated_at: task.updated_at,
                }),
            )
                .into_response(),
            Ok(None) => {
                AppError::NotFound(format!("Task with id {} not found", id)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }

    pub async fn get_tasks_by_section_id_handler(
        state: State<AppState>,
        Path(section_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match TaskService::get_tasks_by_section_id(&state.db, section_id).await {
            Ok(tasks) => {
                let result_data = tasks
                    .iter()
                    .map(|t| TaskDto {
                        id: t.id,
                        name: t.name.clone(),
                        section_id: t.section_id,
                        created_at: t.created_at,
                        updated_at: t.updated_at,
                    })
                    .collect::<Vec<TaskDto>>();
                (StatusCode::OK, Json(result_data)).into_response()
            }
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }
}
