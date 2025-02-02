use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Utc};

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
pub struct TimeEntry {
    pub id: String,
    pub project_id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<i64>, // Duration in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerState {
    pub is_tracking: bool,
    pub current_entry: Option<TimeEntry>,
    pub projects: Vec<Project>,
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

pub fn start_tracking(project_id: String, task_id: String) -> Result<TimeEntry> {
    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e)
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
    };

    state.is_tracking = true;
    state.current_entry = Some(entry.clone());

    Ok(entry)
}

pub fn stop_tracking() -> Result<TimeEntry> {
    let mut state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e)
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
        message: format!("Failed to acquire state lock: {}", e)
    })?;

    Ok(state.current_entry.clone())
}

pub fn get_projects() -> Result<Vec<Project>> {
    let state = TRACKER_STATE.lock().map_err(|e| TrackerError {
        message: format!("Failed to acquire state lock: {}", e)
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
