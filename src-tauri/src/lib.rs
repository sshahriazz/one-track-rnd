// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod keyboard_mouse_activity;
mod screenshot;
mod time_tracker;
use screenshot::ScreenshotConfig;

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global screenshot configuration
pub static SCREENSHOT_CONFIG: Lazy<Mutex<ScreenshotConfig>> =
    Lazy::new(|| Mutex::new(ScreenshotConfig::default()));

// Function to get access to the global config
pub fn global_screenshot_config() -> &'static Mutex<ScreenshotConfig> {
    &SCREENSHOT_CONFIG
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            keyboard_mouse_activity::get_activity_config,
            keyboard_mouse_activity::update_activity_config,
            keyboard_mouse_activity::get_activity_status,
            keyboard_mouse_activity::start_activity_tracking,
            keyboard_mouse_activity::stop_activity_tracking,
            screenshot::get_available_screens,
            screenshot::update_screenshot_config,
            screenshot::get_screenshot_config,
            screenshot::set_capture_mode,
            screenshot::set_quality,
            screenshot::set_file_prefix,
            screenshot::take_screenshots,
            time_tracker::get_all_projects,
            time_tracker::start_time_tracking,
            time_tracker::stop_time_tracking,
            time_tracker::get_active_entry,
            time_tracker::check_idle_status,
            time_tracker::handle_idle_decision,
            time_tracker::is_user_idle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
