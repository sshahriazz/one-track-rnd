use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tokio::time::{Duration, sleep};
use xcap::Monitor;

#[derive(Debug)]
pub struct ScreenShotActivity {
    pub duration: f32,
    pub enabled: bool,
    #[doc(hidden)]
    running: Arc<AtomicBool>,
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

impl ScreenShotActivity {
    pub fn new(duration: f32, enabled: bool) -> Self {
        Self {
            duration,
            enabled,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn update_config(&mut self, duration: f32, enabled: bool) {
        self.duration = duration;
        self.enabled = enabled;
    }

    pub async fn get_config(&self) -> (f32, bool) {
        (self.duration, self.enabled)
    }

    async fn capture_screenshots() {
        println!("Taking screenshots...");
        let start = Instant::now();
        let monitors = Monitor::all().unwrap();

        for monitor in monitors {
            println!("Capturing screenshot from monitor: {}", monitor.name());
            let image = monitor.capture_image().unwrap();

            image
                .save(format!("target/monitor-{}.png", normalized(monitor.name())))
                .unwrap();
        }

        println!("Captured screenshots in: {:?}", start.elapsed());
    }

    pub async fn start_capturing(&self) {
        if !self.enabled {
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let min_duration = self.duration.max(10.0) as u64; // Ensure minimum 10s duration

        println!(
            "Starting periodic screenshot capture with minimum interval of {}s",
            min_duration
        );

        let mut last_capture = Instant::now();
        while running.load(Ordering::SeqCst) {
            // Calculate time since last capture
            let time_since_last = last_capture.elapsed().as_secs();
            println!("Time since last capture: {}s", time_since_last);

            // Take new screenshot and update last capture time
            Self::capture_screenshots().await;
            last_capture = Instant::now();

            // Generate and wait for random delay
            let random_delay = thread_rng().gen_range(5..=min_duration);
            println!("Waiting {}s until next capture...", random_delay);
            sleep(Duration::from_secs(random_delay)).await;
        }

        println!("Screenshot capture stopped");
    }

    pub async fn stop_capturing(&self) {
        println!("Stopping screenshot capture...");
        self.running.store(false, Ordering::SeqCst);
    }
}
