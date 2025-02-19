use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    handlers::task_handlers::TaskMutationHandlers as TMH,
    handlers::task_handlers::TaskQueryHandlers as TQH, AppState,
};

pub fn task_routes() -> Router<AppState> {
    let task_router = Router::new()
        .route("/all", get(TQH::get_tasks_handler))
        .route("/{id}", get(TQH::get_task_by_id_handler))
        .route(
            "/by-section-id/{section_id}",
            get(TQH::get_tasks_by_section_id_handler),
        )
        .route("/create/{section_id}", post(TMH::create_task_handler))
        .route("/update/{id}", patch(TMH::update_task_handler))
        .route("/delete/{id}", get(TMH::delete_task_handler));
    task_router
}
