use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdleTimeEntry {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub project_id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<i64>,            // Duration in seconds
    pub idle_time: Option<IdleTimeEntry>, // Track idle time if any
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerState {
    pub is_tracking: bool,
    pub current_entry: Option<TimeEntry>,
    pub projects: Vec<Project>,
    pub idle_start_time: Option<DateTime<Utc>>,
    pub is_idle: bool,
}

impl Default for TrackerState {
    fn default() -> Self {
        // Create dummy project and tasks for testing
        let tasks = vec![
            Task {
                id: "task1".to_string(),
                name: "Frontend Development".to_string(),
                project_id: "proj1".to_string(),
            },
            Task {
                id: "task2".to_string(),
                name: "Backend API".to_string(),
                project_id: "proj1".to_string(),
            },
        ];

        let project = Project {
            id: "proj1".to_string(),
            name: "One Track App".to_string(),
            tasks,
        };

        Self {
            is_tracking: false,
            current_entry: None,
            projects: vec![project],
            idle_start_time: None,
            is_idle: false,
        }
    }
}

static TRACKER_STATE: Lazy<Mutex<TrackerState>> = Lazy::new(|| Mutex::new(TrackerState::default()));

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerError {
    message: String,
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TrackerError {}

type Result<T> = std::result::Result<T, TrackerError>;

pub fn handle_idle_detection() -> Result<()> {
    use crate::keyboard_mouse_activity::{get_config, track_activity};

    println!("[Backend] Starting idle detection check");

    let config = get_config().map_err(|e| TrackerError {
        message: format!("Failed to get activity config: {}", e),
    })?;

    println!("[Backend] Activity config loaded: {:?}", config);

    if !config.idle_detection_enabled {
        println!("[Backend] Idle detection is disabled");
        return Ok(());
    }

    let activity = track_activity().map_err(|e| TrackerError {
        message: format!("Failed to track activity: {}", e),
    })?;

    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    let is_active = activity.keyboard_active || activity.mouse_active;
    let threshold_duration = chrono::Duration::minutes(config.idle_threshold_minutes as i64);

    println!(
        "[Backend] Activity status - is_active: {}, current_idle_state: {}, idle_start_time: {:?}",
        is_active, state.is_idle, state.idle_start_time
    );

    match (is_active, state.is_idle, state.idle_start_time) {
        // User just became idle
        (false, false, None) => {
            println!("[Backend] User just became inactive, starting idle timer");
            state.idle_start_time = Some(Utc::now());
            state.is_idle = false;
        }
        // User is still inactive and past threshold
        (false, false, Some(start_time)) if Utc::now() - start_time >= threshold_duration => {
            println!(
                "[Backend] User has been inactive for {:?}, marking as idle",
                threshold_duration
            );
            state.is_idle = true;
        }
        // User became active again after being idle
        (true, true, Some(start_time)) => {
            println!("[Backend] User became active again after being idle");
            state.is_idle = false;
            state.idle_start_time = None;
        }
        _ => {
            println!("[Backend] No change in idle state");
        }
    }

    Ok(())
}

pub fn handle_idle_time_decision(keep_time: bool, reason: Option<String>) -> Result<()> {
    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    if let Some(start_time) = state.idle_start_time {
        let end_time = Utc::now();
        let duration = (end_time - start_time).num_seconds();

        if keep_time {
            if let Some(entry) = state.current_entry.as_mut() {
                entry.idle_time = Some(IdleTimeEntry {
                    start_time,
                    end_time,
                    duration,
                    reason,
                });
            }
        }

        state.idle_start_time = None;
        state.is_idle = false;
    }

    Ok(())
}

pub fn start_tracking(project_id: String, task_id: String) -> Result<TimeEntry> {
    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    if state.is_tracking {
        return Err(TrackerError {
            message: "Already tracking time".to_string(),
        });
    }

    let entry = TimeEntry {
        id: uuid::Uuid::new_v4().to_string(),
        project_id,
        task_id,
        start_time: Utc::now(),
        end_time: None,
        duration: None,
        idle_time: None,
    };

    state.is_tracking = true;
    state.current_entry = Some(entry.clone());

    Ok(entry)
}

pub fn stop_tracking() -> Result<TimeEntry> {
    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    if !state.is_tracking {
        return Err(TrackerError {
            message: "No active time tracking".to_string(),
        });
    }

    let mut entry = state.current_entry.take().unwrap();
    let end_time = Utc::now();
    entry.end_time = Some(end_time);
    entry.duration = Some((end_time - entry.start_time).num_seconds());

    state.is_tracking = false;
    state.current_entry = None;

    Ok(entry)
}

pub fn get_current_entry() -> Result<Option<TimeEntry>> {
    let state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    Ok(state.current_entry.clone())
}

pub fn get_projects() -> Result<Vec<Project>> {
    let state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;

    Ok(state.projects.clone())
}

// Tauri Commands

#[tauri::command]
pub async fn get_all_projects() -> Result<Vec<Project>> {
    get_projects()
}

#[tauri::command]
pub async fn start_time_tracking(project_id: String, task_id: String) -> Result<TimeEntry> {
    start_tracking(project_id, task_id)
}

#[tauri::command]
pub async fn stop_time_tracking() -> Result<TimeEntry> {
    stop_tracking()
}

#[tauri::command]
pub async fn get_active_entry() -> Result<Option<TimeEntry>> {
    get_current_entry()
}

#[tauri::command]
pub async fn check_idle_status() -> Result<()> {
    handle_idle_detection()
}

#[tauri::command]
pub async fn handle_idle_decision(keep_time: bool, reason: Option<String>) -> Result<()> {
    handle_idle_time_decision(keep_time, reason)
}

#[tauri::command]
pub async fn is_user_idle() -> Result<bool> {
    let state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e),
    })?;
    Ok(state.is_idle)
}
