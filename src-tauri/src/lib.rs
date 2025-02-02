// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod screenshot;
mod keyboard_mouse_activity;
mod time_tracker;
use screenshot::{
    get_available_screens, get_screenshot_config, set_capture_mode, set_file_prefix, set_quality, take_screenshots, update_screenshot_config, ScreenshotConfig
};
use keyboard_mouse_activity::{
    get_activity_config, update_activity_config, get_activity_status,
    start_activity_tracking, stop_activity_tracking
};
use time_tracker::{
    get_all_projects, start_time_tracking, stop_time_tracking, get_active_entry
};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Global screenshot configuration
pub static SCREENSHOT_CONFIG: Lazy<Mutex<ScreenshotConfig>> = Lazy::new(|| Mutex::new(ScreenshotConfig::default()));

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
