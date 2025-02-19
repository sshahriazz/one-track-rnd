use ot_server::section_dto::SectionDto;
use uuid::Uuid;

use crate::services::section::SectionService;

#[tauri::command]
pub async fn get_sections_by_project_id(project_id: Uuid) -> Result<Vec<SectionDto>, String> {
    let section_service = SectionService::new();
    section_service
        .get_sections_by_project_id(project_id)
        .await
        .map_err(|e| e.to_string())
}
