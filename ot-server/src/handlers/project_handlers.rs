use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_valid::Valid;
use sea_orm::prelude::Uuid;

use crate::{
    dtos::project_dto::{ProjectCreateDto, ProjectDto, ProjectListResponse, ProjectUpdateDto},
    services::project_service::ProjectService,
    utils::error::AppError,
    AppState,
};
pub struct ProjectQueryHandlers;
pub struct ProjectMutationHandlers;

impl ProjectMutationHandlers {
    pub async fn project_create_handler(
        state: State<AppState>,
        Valid(Json(payload)): Valid<Json<ProjectCreateDto>>,
    ) -> impl IntoResponse {
        if !state.is_standalone {
            return (
                StatusCode::BAD_REQUEST,
                AppError::BadRequest(
                    "Standalone mode: Project creation is not allowed".to_string(),
                ),
            )
                .into_response();
        }

        match ProjectService::create_project(&state.db, payload).await {
            Ok(project) => (
                StatusCode::CREATED,
                Json(ProjectDto {
                    id: project.id,
                    name: project.name,
                    version: project.version,
                    created_at: project.created_at,
                    updated_at: project.updated_at,
                }),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }

    pub async fn project_update_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
        Json(payload): Json<ProjectUpdateDto>,
    ) -> impl IntoResponse {
        match ProjectService::update_project(&state.db, id, payload).await {
            Ok(project) => (
                StatusCode::OK,
                Json(ProjectDto {
                    id: project.id,
                    name: project.name,
                    version: project.version,
                    created_at: project.created_at,
                    updated_at: project.updated_at,
                }),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }

    pub async fn project_delete_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match ProjectService::delete_project(&state.db, id).await {
            Ok(_) => (StatusCode::OK, Json(())).into_response(),
            Err(e) => e.into_response(),
        }
    }
}

impl ProjectQueryHandlers {
    pub async fn project_list_handler(state: State<AppState>) -> impl IntoResponse {
        match ProjectService::get_projects(&state.db).await {
            Ok(projects) => {
                let projects = projects
                    .into_iter()
                    .map(|project| ProjectDto {
                        id: project.id,
                        name: project.name,
                        version: project.version,
                        created_at: project.created_at,
                        updated_at: project.updated_at,
                    })
                    .collect::<Vec<_>>();
                (StatusCode::OK, Json(ProjectListResponse { projects })).into_response()
            }
            Err(e) => e.into_response(),
        }
    }
    pub async fn project_by_id_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match ProjectService::get_project_by_id(&state.db, id).await {
            Ok(Some(project)) => (
                StatusCode::OK,
                Json(ProjectDto {
                    id: project.id,
                    name: project.name,
                    version: project.version,
                    created_at: project.created_at,
                    updated_at: project.updated_at,
                }),
            )
                .into_response(),
            Ok(None) => {
                AppError::NotFound(format!("Project with id {} not found", id)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }

    // pub async fn project_with_sections_handler(
    //     state: State<AppState>,
    // ) -> Result<Json<Vec<(project::Model, Vec<section::Model>)>>, StatusCode> {
    //     match PQ::project_with_sections(&state.db).await {
    //         Ok(p) => Ok(Json(p)),
    //         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    //     }
    // }
}
