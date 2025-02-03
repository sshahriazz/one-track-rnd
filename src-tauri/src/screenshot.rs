use crate::global_screenshot_config;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{ImageBuffer, Rgba};
use screenshots::Screen;
use serde::{Deserialize, Serialize};

/// Represents different screenshot capture modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureMode {
    /// Capture a single screen by index
    Single(usize),
    /// Capture all available screens
    All,
    /// Capture multiple selected screens by their indices
    Multiple(Vec<usize>),
}

/// Configuration for screenshot capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotConfig {
    /// The mode of capture
    pub mode: CaptureMode,
    /// JPEG quality (1-100)
    pub quality: u8,
    /// Prefix for the output files
    pub file_prefix: String,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            mode: CaptureMode::All,
            quality: 85,
            file_prefix: "screenshot".to_string(),
        }
    }
}

/// Custom error type for screenshot operations
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotError {
    code: ErrorCode,
    message: String,
}

/// Error codes for different types of failures
#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    NoScreens,
    InvalidScreenIndex,
    CaptureFailed,
    SaveFailed,
    StateLockError,
}

impl std::fmt::Display for ScreenshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ScreenshotError {}

impl ScreenshotError {
    fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

type Result<T> = std::result::Result<T, ScreenshotError>;

/// Information about an available screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    /// Index of the screen
    pub index: usize,
    /// Display name or identifier
    pub name: String,
    /// Width of the captured image
    pub width: u32,
    /// Height of the captured image
    pub height: u32,
    /// Whether this is the primary display
    pub is_primary: bool,
    /// X coordinate of the screen
    pub x: i32,
    /// Y coordinate of the screen
    pub y: i32,
    /// Scale factor of the screen
    pub scale_factor: f32,
    /// Refresh rate of the screen in Hz
    pub frequency: f32,
}

/// Represents a captured screenshot with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedScreenshot {
    /// Index of the screen that was captured
    pub screen_index: usize,
    /// Width of the captured image
    pub width: u32,
    /// Height of the captured image
    pub height: u32,
    /// Base64 encoded JPEG image data
    pub image_data: String,
    /// Whether this is the primary display
    pub is_primary: bool,
    /// Display name
    pub name: String,
    /// X coordinate of the screen
    pub x: i32,
    /// Y coordinate of the screen
    pub y: i32,
    /// Scale factor of the screen
    pub scale_factor: f32,
    /// Refresh rate of the screen in Hz
    pub frequency: f32,
}

/// Get information about all available screens
/// This version can be called from anywhere without Tauri dependencies
pub fn get_screens() -> Result<Vec<ScreenInfo>> {
    println!("Attempting to get all screens...");
    let screens = Screen::all().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::NoScreens,
            format!("Failed to get screens: {}", e),
        )
    })?;
    println!("Found {} screens", screens.len());
    println!("Screen information: {:?}", screens);
    for (i, screen) in screens.iter().enumerate() {
        println!(
            "Screen {}: {}x{} @ ({},{}) [Primary: {}]",
            i,
            screen.display_info.width,
            screen.display_info.height,
            screen.display_info.x,
            screen.display_info.y,
            screen.display_info.is_primary
        );
    }
    Ok(screens
        .iter()
        .enumerate()
        .map(|(index, screen)| ScreenInfo {
            index,
            name: if screen.display_info.is_primary {
                format!("Primary Display ({})", index + 1)
            } else {
                format!("Display {}", index + 1)
            },
            width: screen.display_info.width as u32,
            height: screen.display_info.height as u32,
            is_primary: screen.display_info.is_primary,
            x: screen.display_info.x,
            y: screen.display_info.y,
            scale_factor: screen.display_info.scale_factor as f32,
            frequency: screen.display_info.frequency as f32,
        })
        .collect())
}

/// Get information about all available screens
#[tauri::command]
pub async fn get_available_screens() -> Result<Vec<ScreenInfo>> {
    get_screens()
}

/// Update the screenshot configuration
/// This version can be called from anywhere without Tauri dependencies
pub fn update_config(new_config: ScreenshotConfig) -> Result<()> {
    let mut config = global_screenshot_config().lock().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::StateLockError,
            format!("Failed to acquire config lock: {}", e),
        )
    })?;
    *config = new_config;
    Ok(())
}

/// Update the screenshot configuration
#[tauri::command]
pub async fn update_screenshot_config(config: ScreenshotConfig) -> Result<()> {
    update_config(config)
}

/// Get the current screenshot configuration
/// This version can be called from anywhere without Tauri dependencies
pub fn get_config() -> Result<ScreenshotConfig> {
    let config = global_screenshot_config().lock().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::StateLockError,
            format!("Failed to acquire config lock: {}", e),
        )
    })?;
    Ok(config.clone())
}

/// Get the current screenshot configuration
#[tauri::command]
pub async fn get_screenshot_config() -> Result<ScreenshotConfig> {
    get_config()
}

/// Set the capture mode
/// This version can be called from anywhere without Tauri dependencies
pub fn set_mode(mode: CaptureMode) -> Result<()> {
    let mut config = global_screenshot_config().lock().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::StateLockError,
            format!("Failed to acquire config lock: {}", e),
        )
    })?;
    config.mode = mode;
    Ok(())
}

/// Set the capture mode
#[tauri::command]
pub async fn set_capture_mode(mode: CaptureMode) -> Result<()> {
    set_mode(mode)
}

/// Set the JPEG quality
/// This version can be called from anywhere without Tauri dependencies
pub fn set_jpeg_quality(quality: u8) -> Result<()> {
    let mut config = global_screenshot_config().lock().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::StateLockError,
            format!("Failed to acquire config lock: {}", e),
        )
    })?;
    config.quality = quality.clamp(1, 100);
    Ok(())
}

/// Set the JPEG quality
#[tauri::command]
pub async fn set_quality(quality: u8) -> Result<()> {
    set_jpeg_quality(quality)
}

/// Set the file prefix
/// This version can be called from anywhere without Tauri dependencies
pub fn set_prefix(prefix: String) -> Result<()> {
    let mut config = global_screenshot_config().lock().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::StateLockError,
            format!("Failed to acquire config lock: {}", e),
        )
    })?;
    config.file_prefix = prefix;
    Ok(())
}

/// Set the file prefix
#[tauri::command]
pub async fn set_file_prefix(prefix: String) -> Result<()> {
    set_prefix(prefix)
}

/// Takes screenshots based on the provided configuration
/// This version can be called from anywhere without Tauri dependencies
pub fn capture_screenshots() -> Result<Vec<CapturedScreenshot>> {
    let config = global_screenshot_config()
        .lock()
        .map_err(|e| {
            ScreenshotError::new(
                ErrorCode::StateLockError,
                format!("Failed to acquire config lock: {}", e),
            )
        })?
        .clone();

    println!("Starting screenshot capture process...");

    println!("Getting screen information...");
    let screens = Screen::all().map_err(|e| {
        ScreenshotError::new(
            ErrorCode::CaptureFailed,
            format!("Failed to get screens: {}", e),
        )
    })?;

    println!("Found {} screens", screens.len());
    if screens.is_empty() {
        return Err(ScreenshotError::new(
            ErrorCode::NoScreens,
            "No screens available for capture",
        ));
    }

    let screen_indices = match config.mode {
        CaptureMode::Single(index) => {
            println!("Capturing single screen at index {}", index);
            if index >= screens.len() {
                return Err(ScreenshotError::new(
                    ErrorCode::InvalidScreenIndex,
                    format!(
                        "Screen index {} is out of bounds (max: {})",
                        index,
                        screens.len() - 1
                    ),
                ));
            }
            vec![index]
        }
        CaptureMode::All => {
            println!("Capturing all {} screens", screens.len());
            (0..screens.len()).collect()
        }
        CaptureMode::Multiple(ref indices) => {
            println!("Capturing multiple screens: {:?}", indices);
            for &index in indices {
                if index >= screens.len() {
                    return Err(ScreenshotError::new(
                        ErrorCode::InvalidScreenIndex,
                        format!(
                            "Screen index {} is out of bounds (max: {})",
                            index,
                            screens.len() - 1
                        ),
                    ));
                }
            }
            indices.clone()
        }
    };

    let mut captured_screenshots = Vec::new();

    for &index in &screen_indices {
        println!("Capturing screen {}...", index);
        let screen = &screens[index];
        println!(
            "Screen info: {}x{} @ ({},{})",
            screen.display_info.width,
            screen.display_info.height,
            screen.display_info.x,
            screen.display_info.y
        );

        let buffer = match screen.capture() {
            Ok(buf) => {
                println!("Successfully captured screen {}", index);
                buf
            }
            Err(e) => {
                println!("Failed to capture screen {}: {}", index, e);
                return Err(ScreenshotError::new(
                    ErrorCode::CaptureFailed,
                    format!("Failed to capture screen {}: {}", index, e),
                ));
            }
        };

        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
            buffer.width() as u32,
            buffer.height() as u32,
            buffer.into_raw(),
        )
        .ok_or_else(|| {
            ScreenshotError::new(
                ErrorCode::CaptureFailed,
                format!("Failed to create image buffer for screen {}", index),
            )
        })?;

        // Create an in-memory buffer for the JPEG image
        let mut jpeg_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut jpeg_data);
        img.write_to(&mut cursor, image::ImageFormat::Jpeg)
            .map_err(|e| {
                ScreenshotError::new(
                    ErrorCode::SaveFailed,
                    format!("Failed to encode JPEG for screen {}: {}", index, e),
                )
            })?;

        // Convert the JPEG data to base64
        let base64_data = BASE64.encode(&jpeg_data);

        captured_screenshots.push(CapturedScreenshot {
            screen_index: index,
            width: img.width(),
            height: img.height(),
            image_data: base64_data,
            is_primary: screens[index].display_info.is_primary,
            name: if screens[index].display_info.is_primary {
                format!("Primary Display ({})", index + 1)
            } else {
                format!("Display {}", index + 1)
            },
            x: screens[index].display_info.x,
            y: screens[index].display_info.y,
            scale_factor: screens[index].display_info.scale_factor as f32,
            frequency: screens[index].display_info.frequency as f32,
        });
    }

    if captured_screenshots.is_empty() {
        Err(ScreenshotError::new(
            ErrorCode::CaptureFailed,
            "No screenshots were captured",
        ))
    } else {
        Ok(captured_screenshots)
    }
}

/// Takes screenshots based on the current configuration in state
#[tauri::command]
pub async fn take_screenshots() -> Result<Vec<CapturedScreenshot>> {
    capture_screenshots()
}
