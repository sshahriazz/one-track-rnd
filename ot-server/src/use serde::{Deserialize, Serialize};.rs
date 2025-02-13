use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ActivityData {
    id: String,
    screenshots: Vec<String>,
    timestamp: String,
    keyboard_activity_percent: String,
    mouse_activity_percent: String,
    total_percent: String,
    track_interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WindowActivityData {
    id: String,
    active_window_ss: Vec<String>,
    active_window_data: ActiveWindowData,
}

#[derive(Debug, Deserialize, Serialize)]
struct ActiveWindowData {
    name: String,
    position: String,
    active_duration: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BrowserActivityData {
    id: String,
    visited_url: String,
    screenshot: String,
    active_duration: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Activity {
    id: String,
    tracked_time: String,
    total_activity_percentage: String,
    session_start: String,
    session_end: String,
    activity_data: Vec<ActivityData>,
    window_activity_data: Vec<WindowActivityData>,
    browser_activity_data: Vec<BrowserActivityData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    data: Vec<Activity>,
}
