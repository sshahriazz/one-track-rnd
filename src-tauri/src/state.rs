use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// Represents the state of a timer with elapsed time tracking and timestamps

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub elapsed: Duration,
    pub running: bool,
    #[serde(skip)]
    pub start_instant: Option<Instant>,
    pub start_date_time: Option<DateTime<Utc>>,
    pub end_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // screenshots configs
    pub enable_screen_shots: bool,
    pub screen_shots_interval: Duration,
    pub enable_window_ss: bool,
    pub browser_window_ss: bool,
    // activity configs
    pub enable_keyboard_activity: bool,
    pub enable_mouse_activity: bool,
    // idle configs
    pub enable_idle_time: bool,
    pub idle_time_threshold: Duration,
    pub ask_for_idle_reason: bool,
    // window configs
    pub enable_window_activity: bool,
    // browser configs
    pub enable_browser_activity: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enable_screen_shots: true,
            screen_shots_interval: Duration::from_secs(10),
            enable_window_ss: true,
            browser_window_ss: false,
            enable_keyboard_activity: true,
            enable_mouse_activity: true,
            enable_idle_time: true,
            idle_time_threshold: Duration::from_secs(30),
            ask_for_idle_reason: true,
            enable_window_activity: true,
            enable_browser_activity: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub timer_state: TimerState,
    pub app_config: AppConfig,
}
