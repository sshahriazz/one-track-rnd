use ot_server::task_dto::TaskDto;
use uuid::Uuid;

use crate::services::task::TaskService;

#[tauri::command]
pub async fn get_tasks_by_section_id(section_id: Uuid) -> Result<Vec<TaskDto>, String> {
    let task_service = TaskService::new();
    task_service
        .get_tasks_by_section_id(section_id)
        .await
        .map_err(|e| e.to_string())
}
