use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{handlers::sub_task_handlers::SubTaskHandlers, AppState};

pub fn sub_task_routes() -> Router<AppState> {
    Router::new()
        .route("/all", get(SubTaskHandlers::get_sub_tasks_handler))
        .route("/{id}", get(SubTaskHandlers::get_sub_task_by_id_handler))
        .route(
            "/by-section-id/{section_id}",
            get(SubTaskHandlers::get_sub_tasks_by_section_id_handler),
        )
        .route(
            "/by-task-id/{task_id}",
            get(SubTaskHandlers::get_sub_tasks_by_task_id_handler),
        )
        .route(
            "/create/{section_id}/{task_id}",
            post(SubTaskHandlers::create_sub_task_handler),
        )
        .route(
            "/update/{id}",
            patch(SubTaskHandlers::update_sub_task_handler),
        )
        .route(
            "/delete/{id}",
            get(SubTaskHandlers::delete_sub_task_handler),
        )
}
