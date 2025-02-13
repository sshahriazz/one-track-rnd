use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    handlers::section_handlers::SectionMutationHandlers as SMH,
    handlers::section_handlers::SectionQueryHandlers as SQH, AppState,
};

pub fn section_routes() -> Router<AppState> {
    let section_router = Router::new()
        .route("/all", get(SQH::get_sections_handler))
        .route("/{id}", get(SQH::get_section_by_id_handler))
        .route("/create", post(SMH::create_section_handler))
        .route("/update/{id}", patch(SMH::update_section_handler))
        .route("/delete/{id}", get(SMH::delete_section_handler));
    section_router
}
