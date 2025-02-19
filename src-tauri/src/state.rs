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

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub timer_state: TimerState,
}
