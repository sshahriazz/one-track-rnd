use std::sync::{Arc, Mutex, atomic::AtomicBool};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, State};
use tokio::time::{Duration, Instant};

use crate::{
    AppState,
    track_activity::{ScreenShotCommand, screenshot_command},
};

/// Commands that can be sent to control the timer
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TimerCommand {
    /// Start the timer
    Start,
    /// Pause the timer, preserving elapsed time
    Pause,
    /// Resume the timer from its paused state
    Resume,
    /// Stop the timer and reset elapsed time
    Stop,
    /// Add a specific number of seconds to the timer
    AddTime(u64),
}

/// Error type for timer operations
#[derive(Debug, thiserror::Error, Serialize)]
#[allow(dead_code)]
pub enum TimerError {
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
    #[error("Failed to emit event: {0}")]
    EmitError(String),
}

#[tauri::command()]
pub async fn control_timer(
    state: State<'_, Mutex<AppState>>,
    command: TimerCommand,
    app_handle: tauri::AppHandle,
) -> Result<(), TimerError> {
    let mut app_state = state
        .lock()
        .map_err(|e| TimerError::LockError(format!("{:?}", e)))?;
    match command {
        TimerCommand::Start => {
            let should_start =
                !app_state.timer_state.running && app_state.timer_state.elapsed.as_secs() == 0;

            if should_start {
                // Reset and update timer state
                app_state.timer_state.running = true;
                app_state.timer_state.start_instant = Some(Instant::now());
                app_state.timer_state.start_date_time = Some(Utc::now());
                app_state.timer_state.end_date_time = None;
                app_state.timer_state.elapsed = Duration::from_secs(0);
                
                // Always enable screenshots when starting fresh
                app_state.app_config.enable_screen_shots = true;
                let enable_screen_shots = app_state.app_config.enable_screen_shots;
                let idle_time_threshold = app_state.app_config.screen_shots_interval;
                
                println!(
                    "[Timer] Starting timer. Screen shots: {:?}, Idle threshold: {:?}",
                    enable_screen_shots, idle_time_threshold
                );
                
                tokio::spawn(async move {
                    screenshot_command(
                        ScreenShotCommand::Running,
                        Some(idle_time_threshold),
                        Some(enable_screen_shots),
                    )
                    .await;
                });
            }

            // Drop the lock before async operation
        }
        TimerCommand::Pause => {
            // Pause should only work if timer is running
            if app_state.timer_state.running {
                if let Some(start) = app_state.timer_state.start_instant {
                    app_state.timer_state.elapsed += start.elapsed();
                }
                app_state.timer_state.running = false;
                app_state.timer_state.start_instant = None;
                app_state.app_config.enable_screen_shots = false;
                tokio::spawn(async move {
                    screenshot_command(ScreenShotCommand::Stopped, None, None).await;
                });
            }
        }
        TimerCommand::Resume => {
            // Resume should only work if timer is not running and has elapsed time
            let should_resume =
                !app_state.timer_state.running && app_state.timer_state.elapsed.as_secs() > 0;
            if should_resume {
                app_state.timer_state.running = true;
                app_state.timer_state.start_instant = Some(Instant::now());
                app_state.app_config.enable_screen_shots = true;
                let enable_screen_shots = app_state.app_config.enable_screen_shots;
                let idle_time_threshold = app_state.app_config.screen_shots_interval;
                tokio::spawn(async move {
                    screenshot_command(
                        ScreenShotCommand::Running,
                        Some(idle_time_threshold),
                        Some(enable_screen_shots),
                    )
                    .await;
                });
            }
        }
        TimerCommand::Stop => {
            // Stop should accumulate elapsed time if running, then reset
            if app_state.timer_state.running {
                if let Some(start) = app_state.timer_state.start_instant {
                    app_state.timer_state.elapsed += start.elapsed();
                }
                tokio::spawn(async move {
                    screenshot_command(ScreenShotCommand::Stopped, None, None).await;
                });
            }
            // Record end time before resetting
            app_state.timer_state.end_date_time = Some(Utc::now());

            // Reset timer state
            app_state.timer_state.running = false;
            app_state.timer_state.start_instant = None;
            app_state.timer_state.elapsed = Duration::from_secs(0);
            app_state.app_config.enable_screen_shots = false;

            // Emit one final update after stopping
            let timer_response = TimerResponse {
                elapsed_seconds: 0,
                running: false,
                start_time: None,
                end_time: app_state.timer_state.end_date_time,
            };
            if let Err(e) = app_handle.emit("timer-update", timer_response) {
                eprintln!("Failed to emit final timer update: {:?}", e);
            }
        }
        TimerCommand::AddTime(seconds) => {
            // Add time should only work when timer is paused (not running but has elapsed time)
            if !app_state.timer_state.running && app_state.timer_state.elapsed.as_secs() > 0 {
                app_state.timer_state.elapsed += Duration::from_secs(seconds);
            }
        }
    }
    Ok(())
}

/// Response type for timer state information
#[derive(Debug, Serialize, Clone)]
pub struct TimerResponse {
    elapsed_seconds: u64,
    running: bool,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
}

/// Get the current timer state
fn get_current_timer_state(state: &Mutex<AppState>) -> Result<TimerResponse, TimerError> {
    let app_state = state
        .lock()
        .map_err(|e| TimerError::LockError(format!("{:?}", e)))?;
    let mut total_elapsed = app_state.timer_state.elapsed;

    if app_state.timer_state.running {
        if let Some(start) = app_state.timer_state.start_instant {
            total_elapsed += start.elapsed();
        }
    }

    Ok(TimerResponse {
        elapsed_seconds: total_elapsed.as_secs(),
        running: app_state.timer_state.running,
        start_time: app_state.timer_state.start_date_time,
        end_time: app_state.timer_state.end_date_time,
    })
}

/// Start a background task that emits timer updates
#[tauri::command]
pub async fn start_timer_updates(app: tauri::AppHandle) -> Result<(), TimerError> {
    // Create a cancellation token
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

    // Store the sender in app state for cleanup
    app.manage(tx);

    // Spawn a background task
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // Check for cancellation
                _ = rx.recv() => {
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    // Get the managed state
                    let state = app.state::<Mutex<AppState>>();
                    if let Ok(timer_response) = get_current_timer_state(state.inner()) {
                        // Always emit timer updates to ensure frontend stays in sync
                        if let Err(e) = app.emit("timer-update", timer_response) {
                            eprintln!("Failed to emit timer update: {:?}", e);
                        }
                    }
                }
            }
        }
    });

    Ok(())
}
