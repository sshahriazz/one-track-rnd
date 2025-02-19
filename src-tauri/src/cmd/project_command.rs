use ot_server::project_dto::ProjectDto;

use crate::services::project::ProjectService;

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn get_projects() -> Result<Vec<ProjectDto>, String> {
    let project_service = ProjectService::new();
    project_service
        .get_projects()
        .await
        .map_err(|e| e.to_string())
}
