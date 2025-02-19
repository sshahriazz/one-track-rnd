use state::AppState;
use std::sync::Mutex;
use tauri::Manager;

mod cmd;
mod services;
mod state;
mod task_timer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            task_timer::control_timer,
            task_timer::start_timer_updates,
            cmd::project_command::get_projects,
            cmd::section_command::get_sections_by_project_id,
            cmd::sub_task_command::get_sub_tasks_by_task_id,
            cmd::task_command::get_tasks_by_section_id
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
