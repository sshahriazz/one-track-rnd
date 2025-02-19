use ot_server::sub_task_dto::SubTaskDto;
use uuid::Uuid;

use crate::services::sub_task::SubTaskService;

#[tauri::command]
pub async fn get_sub_tasks_by_task_id(task_id: Uuid) -> Result<Vec<SubTaskDto>, String> {
    let sub_task_service = SubTaskService::new();
    sub_task_service
        .get_sub_tasks_by_task_id(task_id)
        .await
        .map_err(|e| e.to_string())
}
