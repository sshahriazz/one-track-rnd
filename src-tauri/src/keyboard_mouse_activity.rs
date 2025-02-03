use device_query::{DeviceQuery, DeviceState, MouseState};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;
use std::{thread, time::Duration};

/// Configuration for activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityConfig {
    /// Enable/disable keyboard tracking
    pub track_keyboard: bool,
    /// Enable/disable mouse tracking
    pub track_mouse: bool,
    /// Whether tracking is currently active
    pub is_tracking: bool,
}

impl Default for ActivityConfig {
    fn default() -> Self {
        Self {
            track_keyboard: true,
            track_mouse: true,
            is_tracking: false,
        }
    }
}

/// Activity tracking results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityStatus {
    /// Whether keyboard activity was detected
    pub keyboard_active: bool,
    /// Whether mouse activity was detected
    pub mouse_active: bool,
}

static DEVICE_STATE: Lazy<DeviceState> = Lazy::new(|| DeviceState::new());
static LAST_MOUSE_POS: Lazy<Mutex<(i32, i32)>> = Lazy::new(|| Mutex::new((0, 0)));
static CONFIG: Lazy<Mutex<ActivityConfig>> = Lazy::new(|| Mutex::new(ActivityConfig::default()));
static PREV_KEYS: Lazy<Mutex<HashSet<device_query::Keycode>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Custom error type for activity tracking
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityError {
    message: String,
}

impl std::fmt::Display for ActivityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ActivityError {}

type Result<T> = std::result::Result<T, ActivityError>;

/// Track keyboard activity
pub fn track_keyboard() -> bool {
    // Add a small sleep to avoid excessive polling
    thread::sleep(Duration::from_millis(10));
    
    let current_keys: HashSet<_> = DEVICE_STATE.get_keys().into_iter().collect();
    let mut prev_keys = PREV_KEYS.lock().unwrap();
    
    let has_activity = current_keys != *prev_keys;
    *prev_keys = current_keys;
    
    has_activity
}

/// Track mouse activity
pub fn track_mouse() -> bool {
    let mouse: MouseState = DEVICE_STATE.get_mouse();
    let current_pos = mouse.coords;
    let mut last_pos = LAST_MOUSE_POS.lock().unwrap();
    
    let moved = current_pos != *last_pos;
    let clicked = mouse.button_pressed.iter().any(|&b| b);
    
    // Only update last position if the mouse actually moved
    if moved {
        *last_pos = current_pos;
    }
    
    moved || clicked
}

/// Start activity tracking
pub fn start_tracking() -> Result<()> {
    let mut config = CONFIG.lock().map_err(|e| ActivityError {
        message: format!("Failed to acquire config lock: {}", e),
    })?;
    config.is_tracking = true;
    Ok(())
}

/// Stop activity tracking
pub fn stop_tracking() -> Result<()> {
    let mut config = CONFIG.lock().map_err(|e| ActivityError {
        message: format!("Failed to acquire config lock: {}", e),
    })?;
    config.is_tracking = false;
    Ok(())
}

/// Combined tracking based on configuration
pub fn track_activity() -> Result<ActivityStatus> {
    let config = CONFIG.lock().map_err(|e| ActivityError {
        message: format!("Failed to acquire config lock: {}", e),
    })?;

    if !config.is_tracking {
        return Ok(ActivityStatus {
            keyboard_active: false,
            mouse_active: false,
        });
    }

    let keyboard_active = if config.track_keyboard {
        track_keyboard()
    } else {
        false
    };
    let mouse_active = if config.track_mouse {
        track_mouse()
    } else {
        false
    };

    Ok(ActivityStatus {
        keyboard_active,
        mouse_active,
    })
}

/// Update the activity tracking configuration
pub fn update_config(new_config: ActivityConfig) -> Result<()> {
    let mut config = CONFIG.lock().map_err(|e| ActivityError {
        message: format!("Failed to acquire config lock: {}", e),
    })?;
    *config = new_config;
    Ok(())
}

/// Get the current activity tracking configuration
pub fn get_config() -> Result<ActivityConfig> {
    let config = CONFIG.lock().map_err(|e| ActivityError {
        message: format!("Failed to acquire config lock: {}", e),
    })?;
    Ok(config.clone())
}

// Tauri commands

#[tauri::command]
pub async fn get_activity_config() -> Result<ActivityConfig> {
    get_config()
}

#[tauri::command]
pub async fn update_activity_config(config: ActivityConfig) -> Result<()> {
    update_config(config)
}

#[tauri::command]
pub async fn get_activity_status() -> Result<ActivityStatus> {
    track_activity()
}

#[tauri::command]
pub async fn start_activity_tracking() -> Result<()> {
    start_tracking()
}

#[tauri::command]
pub async fn stop_activity_tracking() -> Result<()> {
    stop_tracking()
}
