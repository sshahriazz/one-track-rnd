use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    handlers::project_handlers::{ProjectMutationHandlers as PMH, ProjectQueryHandlers as PQH},
    AppState,
};

pub fn project_routes() -> Router<AppState> {
    let project_router = Router::new()
        .route("/all", get(PQH::project_list_handler))
        .route("/{id}", get(PQH::project_by_id_handler))
        .route("/create", post(PMH::project_create_handler))
        .route("/update/{id}", patch(PMH::project_update_handler))
        .route("/delete/{id}", get(PMH::project_delete_handler));
    project_router
}
