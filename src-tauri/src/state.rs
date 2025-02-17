use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;
use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// Represents the state of a timer with elapsed time tracking and timestamps

#[derive(Debug, Default, Clone)]
pub struct TimerState {
    pub elapsed: Duration,
    pub running: bool,
    pub start_instant: Option<Instant>,
    pub start_date_time: Option<DateTime<Utc>>,
    pub end_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone)]

pub struct Project {
    pub id: Uuid,
    pub sections: Vec<Uuid>, // Store references instead of whole objects
}
#[derive(Debug, Default, Clone)]

pub struct Section {
    pub id: Uuid,
    pub tasks: Vec<Uuid>,     // Store references instead of whole objects
    pub sub_tasks: Vec<Uuid>, // Store references instead of whole objects
}
#[derive(Debug, Default, Clone)]

pub struct Task {
    pub id: Uuid,
    pub timer_state: Vec<TimerState>,
    pub sub_tasks: Vec<Uuid>, // Store references instead of whole objects
}
#[derive(Debug, Default, Clone)]
pub struct SubTask {
    pub id: Uuid,
    pub timer_state: Vec<TimerState>,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub timer_state: TimerState,
    pub projects: HashMap<Uuid, Project>, // Store objects in hashmaps for O(1) lookup
    pub sections: HashMap<Uuid, Section>,
    pub tasks: HashMap<Uuid, Task>,
    pub sub_tasks: HashMap<Uuid, SubTask>,
}
