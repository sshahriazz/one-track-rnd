use lazy_static::lazy_static;
use rand::Rng;
use rand::thread_rng;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref SCREENSHOT_RUNNING: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);
}
use std::time::Instant;
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use tokio::time::{Duration, sleep};
use xcap::Monitor;
use xcap::XCapError;

use crate::state::AppState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScreenShotCommand {
    Running,
    Stopped,
}

#[derive(Debug, thiserror::Error, Serialize)]
#[allow(dead_code)]
pub enum ActivityError {
    #[error("Failed to acquire lock: {0}")]
    LockError(String),
    #[error("Failed to emit event: {0}")]
    EmitError(String),
    #[error("Failed to get monitors: {0}")]
    MonitorError(String),
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub async fn screenshot_command(
    command: ScreenShotCommand,
    duration: Option<Duration>,
    enabled: Option<bool>,
) {
    println!("Enabled: {:?}", enabled);
    println!("[Screenshot] Command received: {:?}", command);
    println!(
        "[Screenshot] Duration: {:?}, Enabled: {:?}",
        duration, enabled
    );
    match command {
        ScreenShotCommand::Running => {
            if !enabled.unwrap_or(false) {
                println!("[Screenshot] Screenshots disabled, exiting...");
                return;
            }

            // Stop any existing screenshot service
            if let Ok(mut running) = SCREENSHOT_RUNNING.lock() {
                if let Some(flag) = running.take() {
                    flag.store(false, Ordering::SeqCst);
                    println!("[Screenshot] Stopping existing service...");
                }
            }

            println!("[Screenshot] Starting screenshot service...");
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = running.clone();

            // Store the running flag in static state
            if let Ok(mut screenshot_running) = SCREENSHOT_RUNNING.lock() {
                *screenshot_running = Some(running_clone);
            }

            tokio::spawn(async move {
                // Generate initial random delay within the duration window
                let max_interval = duration
                    .unwrap_or(Duration::from_secs(10))
                    .as_secs()
                    .max(10);

                while running.load(Ordering::SeqCst) {
                    // Generate a random delay between 5s and max_interval
                    let random_delay =
                        Duration::from_secs(thread_rng().gen_range(5..=max_interval));
                    println!(
                        "[Screenshot] Waiting {}s until next capture...",
                        random_delay.as_secs()
                    );
                    sleep(random_delay).await;

                    // Only proceed if we're still running after the delay
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }

                    println!("[Screenshot] Taking new screenshots...");
                    if let Some(monitors) = get_all_monitors().await {
                        println!("[Screenshot] Found {} monitors", monitors.len());
                        for (i, monitor) in monitors.iter().enumerate() {
                            println!("[Screenshot] Processing monitor {}", i + 1);
                            if let Ok(screenshot) = monitor.capture_image() {
                                let timestamp =
                                    chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
                                let filename = format!("screenshot_{}.png", normalized(&timestamp));
                                println!("[Screenshot] Saving screenshot to: {}", filename);

                                // Save screenshot asynchronously
                                tokio::spawn(async move {
                                    if let Err(e) = screenshot.save(&filename) {
                                        eprintln!(
                                            "[Screenshot] Error: Failed to save screenshot {}: {}",
                                            filename, e
                                        );
                                    }
                                });
                            }
                        }
                    }
                }
            });
        }
        ScreenShotCommand::Stopped => {
            println!("[Screenshot] Stopping screenshot service...");
            if let Ok(mut running) = SCREENSHOT_RUNNING.lock() {
                if let Some(flag) = running.take() {
                    flag.store(false, Ordering::SeqCst);
                    println!("[Screenshot] Service stopped successfully");
                } else {
                    println!("[Screenshot] No running service found");
                }
            }
        }
    }
}

async fn get_all_monitors() -> Option<Vec<Monitor>> {
    match Monitor::all() {
        Ok(m) => Some(m),
        Err(e) => {
            eprintln!("Error fetching monitors: {}", e);
            None
        }
    }
}

// pub async fn start_capturing(duration: Duration, enabled: bool) {
//     println!(
//         "Starting screenshot capture... {:?} {:?}",
//         duration, enabled
//     );
//     if !enabled {
//         return;
//     }

//     let running = Arc::new(AtomicBool::new(true));
//     let min_duration = duration.as_secs().max(10);

//     println!(
//         "Starting periodic screenshot capture with minimum interval of {}s",
//         min_duration
//     );

//     let mut last_capture = Instant::now();
//     while running.load(Ordering::SeqCst) {
//         // Calculate time since last capture
//         let time_since_last = last_capture.elapsed().as_secs();
//         println!("Time since last capture: {}s", time_since_last);

//         // Take new screenshot and update last capture time
//         capture_screenshots().await;
//         last_capture = Instant::now();

//         // Generate and wait for random delay
//         let random_delay = thread_rng().gen_range(5..=min_duration);
//         println!("Waiting {}s until next capture...", random_delay);
//         sleep(Duration::from_secs(random_delay)).await;
//     }

//     println!("Screenshot capture stopped");
// }

// async fn get_all_monitors() -> Option<Vec<Monitor>> {
//     match Monitor::all() {
//         Ok(m) => Some(m),
//         Err(e) => {
//             println!("Error fetching monitors: {}", e);
//             None
//         }
//     }
// }

// async fn capture_screenshots() {
//     println!("Taking screenshots...");
//     let start = Instant::now();

//     // Get all monitors
//     let monitors = match get_all_monitors().await {
//         Some(monitors) => monitors,
//         None => {
//             println!("Monitors not found");
//             return;
//         }
//     };

//     println!("monitors: {:?}", monitors);
//     // Take screenshots from each monitor
//     for monitor in monitors {
//         println!("Capturing screenshot from monitor: {}", monitor.name());
//         let image = monitor.capture_image().unwrap();

//         image
//             .save(format!("target/monitor-{}.png", normalized(monitor.name())))
//             .unwrap();
//     }

//     println!("Captured screenshots in: {:?}", start.elapsed());
// }
