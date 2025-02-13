use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_valid::Valid;
use reqwest::StatusCode;
use sea_orm::prelude::Uuid;

use crate::{
    dtos::section_dto::{SectionCreateDto, SectionDto, SectionUpdateDto},
    services::section_service::SectionService,
    utils::error::AppError,
    AppState,
};

pub struct SectionMutationHandlers;
pub struct SectionQueryHandlers;

impl SectionMutationHandlers {
    pub async fn create_section_handler(
        state: State<AppState>,
        Valid(Json(payload)): Valid<Json<SectionCreateDto>>,
    ) -> impl IntoResponse {
        match SectionService::create_section(&state.db, payload).await {
            Ok(section) => (
                StatusCode::OK,
                Json(SectionDto {
                    id: section.id,
                    name: section.name,
                    created_at: section.created_at,
                    updated_at: section.updated_at,
                }),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }

    pub async fn update_section_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
        Valid(Json(payload)): Valid<Json<SectionUpdateDto>>,
    ) -> impl IntoResponse {
        match SectionService::update_section(&state.db, id, payload).await {
            Ok(section) => (
                StatusCode::OK,
                Json(SectionDto {
                    id: section.id,
                    name: section.name,
                    created_at: section.created_at,
                    updated_at: section.updated_at,
                }),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }

    pub async fn delete_section_handler(
        state: State<AppState>,
        Path(section_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match SectionService::delete_section(&state.db, section_id).await {
            Ok(section) => (
                StatusCode::OK,
                format!("Section with id {} deleted", section_id),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }
}

impl SectionQueryHandlers {
    pub async fn get_section_by_id_handler(
        state: State<AppState>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        match SectionService::get_section_by_id(&state.db, id).await {
            Ok(Some(section)) => (
                StatusCode::OK,
                Json(SectionDto {
                    id: section.id,
                    name: section.name,
                    created_at: section.created_at,
                    updated_at: section.updated_at,
                }),
            )
                .into_response(),
            Ok(None) => {
                AppError::NotFound(format!("Section with id {} not found", id)).into_response()
            }
            Err(e) => e.into_response(),
        }
    }

    pub async fn get_sections_handler(state: State<AppState>) -> impl IntoResponse {
        match SectionService::get_sections(&state.db).await {
            Ok(sections) => (
                StatusCode::OK,
                Json(
                    sections
                        .iter()
                        .map(|s| SectionDto {
                            id: s.id,
                            name: s.name.clone(),
                            created_at: s.created_at,
                            updated_at: s.updated_at,
                        })
                        .collect::<Vec<SectionDto>>(),
                ),
            )
                .into_response(),
            Err(e) => e.into_response(),
        }
    }
}
