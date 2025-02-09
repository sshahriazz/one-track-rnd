import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ActivityTracker } from "./components/ActivityTracker";
import "./App.css";

interface ScreenInfo {
  index: number;
  name: string;
  width: number;
  height: number;
}

interface CapturedScreenshot {
  screen_index: number;
  width: number;
  height: number;
  image_data: string;
}

interface Project {
  id: string;
  name: string;
  tasks: Task[];
}

interface Task {
  id: string;
  name: string;
  project_id: string;
}

interface ActivityConfig {
  track_keyboard: boolean;
  track_mouse: boolean;
  is_tracking: boolean;
  idle_detection_enabled: boolean;
  idle_threshold_minutes: number;
  require_idle_reason: boolean;
}

interface IdleTimeEntry {
  start_time: string;
  end_time: string;
  duration: number;
  reason?: string;
}

interface TimeEntry {
  id: string;
  project_id: string;
  task_id: string;
  start_time: string;
  end_time?: string;
  duration?: number;
  idle_time?: IdleTimeEntry;
}

function App() {
  const [screens, setScreens] = useState<ScreenInfo[]>([]);
  const [selectedScreens, setSelectedScreens] = useState<number[]>([]);
  const [quality, setQuality] = useState(85);
  const [filePrefix, setFilePrefix] = useState("screenshot");
  const [status, setStatus] = useState("");
  const [capturedScreenshots, setCapturedScreenshots] = useState<
    CapturedScreenshot[]
  >([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<string>("");
  const [selectedTask, setSelectedTask] = useState<string>("");
  const [isTracking, setIsTracking] = useState(false);
  const [activeEntry, setActiveEntry] = useState<TimeEntry | null>(null);
  const [elapsedTime, setElapsedTime] = useState<string>("00:00:00");
  const [showIdleDialog, setShowIdleDialog] = useState(false);
  const [idleReason, setIdleReason] = useState("");
  const [isIdle, setIsIdle] = useState(false);
  const [activityConfig, setActivityConfig] = useState<ActivityConfig>({
    track_keyboard: true,
    track_mouse: true,
    is_tracking: false,
    idle_detection_enabled: true,
    idle_threshold_minutes: 5,
    require_idle_reason: true,
  });

  useEffect(() => {
    loadScreens();
    loadProjects();
    checkActiveEntry();
    loadActivityConfig();

    // Set up idle detection check
    console.log("Setting up idle detection with config:", activityConfig);

    const idleCheckInterval = setInterval(async () => {
      console.log(
        "Checking idle status. isTracking:",
        isTracking,
        "idle detection enabled:",
        activityConfig.idle_detection_enabled
      );

      if (isTracking && activityConfig.idle_detection_enabled) {
        try {
          await invoke("check_idle_status");
          const userIsIdle = await invoke<boolean>("is_user_idle");
          console.log(
            "User idle status:",
            userIsIdle,
            "previous idle state:",
            isIdle
          );

          if (userIsIdle && !isIdle) {
            console.log("User became idle");
            setIsIdle(true);
          } else if (!userIsIdle && isIdle) {
            console.log("User became active, showing idle dialog");
            setIsIdle(false);
            setShowIdleDialog(true);
          }
        } catch (error) {
          console.error("Error checking idle status:", error);
        }
      }
    }, 10000); // Check every 10 seconds for testing

    return () => {
      clearInterval(idleCheckInterval);
    };
  }, [isTracking, isIdle]);

  useEffect(() => {
    let interval: number | undefined;
    if (isTracking && activeEntry) {
      interval = window.setInterval(() => {
        const startTime = new Date(activeEntry.start_time).getTime();
        const now = new Date().getTime();
        const diff = Math.floor((now - startTime) / 1000);

        const hours = Math.floor(diff / 3600);
        const minutes = Math.floor((diff % 3600) / 60);
        const seconds = diff % 60;

        setElapsedTime(
          `${hours.toString().padStart(2, "0")}:${minutes
            .toString()
            .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`
        );
      }, 1000);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [isTracking, activeEntry]);

  async function loadScreens() {
    try {
      const availableScreens = await invoke<ScreenInfo[]>(
        "get_available_screens"
      );
      setScreens(availableScreens);
    } catch (error) {
      setStatus("Failed to load screens: " + error);
    }
  }

  async function loadActivityConfig() {
    try {
      const config = await invoke<ActivityConfig>("get_activity_config");
      setActivityConfig(config);
    } catch (error) {
      console.error("Error loading activity config:", error);
    }
  }

  const handleConfigChange = async (updates: Partial<ActivityConfig>) => {
    try {
      const newConfig = { ...activityConfig, ...updates };
      await invoke("update_activity_config", { config: newConfig });
      setActivityConfig(newConfig);
    } catch (error) {
      console.error("Error updating activity config:", error);
    }
  };

  async function loadProjects() {
    try {
      const projectList = await invoke<Project[]>("get_all_projects");
      setProjects(projectList);
      if (projectList.length > 0) {
        setSelectedProject(projectList[0].id);
        if (projectList[0].tasks.length > 0) {
          setSelectedTask(projectList[0].tasks[0].id);
        }
      }
    } catch (error) {
      console.error("Error loading projects:", error);
    }
  }

  async function checkActiveEntry() {
    try {
      const entry = await invoke<TimeEntry | null>("get_active_entry");
      if (entry) {
        setIsTracking(true);
        setActiveEntry(entry);
        setSelectedProject(entry.project_id);
        setSelectedTask(entry.task_id);
      }
    } catch (error) {
      console.error("Error checking active entry:", error);
    }
  }

  async function takeScreenshot() {
    try {
      setStatus("Taking screenshot...");
      setCapturedScreenshots([]);

      await invoke("update_screenshot_config", {
        config: {
          mode:
            selectedScreens.length === 0
              ? { All: null }
              : { Multiple: selectedScreens },
          quality,
          file_prefix: filePrefix,
        },
      });

      const screenshots = await invoke<CapturedScreenshot[]>(
        "take_screenshots"
      );
      setCapturedScreenshots(screenshots);
      setStatus(`Successfully captured ${screenshots.length} screenshot(s)`);
    } catch (error) {
      setStatus("Failed to take screenshot: " + error);
    }
  }

  const toggleScreen = (index: number) => {
    setSelectedScreens((prev) =>
      prev.includes(index) ? prev.filter((i) => i !== index) : [...prev, index]
    );
  };

  const handleStartTracking = async () => {
    if (!selectedProject || !selectedTask) return;

    try {
      console.log("Starting time tracking and activity monitoring");

      // Start activity tracking first
      await invoke("start_activity_tracking");

      // Then start time tracking
      const entry = await invoke<TimeEntry>("start_time_tracking", {
        projectId: selectedProject,
        taskId: selectedTask,
      });

      console.log("Time tracking started with entry:", entry);
      setIsTracking(true);
      setActiveEntry(entry);
      setElapsedTime("00:00:00");
    } catch (error) {
      console.error("Error starting time tracking:", error);
    }
  };

  const handleStopTracking = async () => {
    try {
      console.log("Stopping time tracking and activity monitoring");

      // Stop time tracking first
      await invoke<TimeEntry>("stop_time_tracking");

      // Then stop activity tracking
      await invoke("stop_activity_tracking");

      console.log("Time tracking stopped");
      setIsTracking(false);
      setActiveEntry(null);
      setElapsedTime("00:00:00");
    } catch (error) {
      console.error("Error stopping time tracking:", error);
    }
  };

  const handleIdleDecision = async (keepTime: boolean) => {
    try {
      await invoke("handle_idle_decision", {
        keepTime,
        reason:
          keepTime && activityConfig.require_idle_reason
            ? idleReason
            : undefined,
      });
      setShowIdleDialog(false);
      setIdleReason("");
    } catch (error) {
      console.error("Error handling idle decision:", error);
    }
  };

  return (
    <div className="app-container">
      <h1 className="app-title">Screen Capture</h1>

      {/* Activity Tracker */}
      <ActivityTracker />

      {/* Activity Configuration */}
      <div className="section">
        <h2 className="section-title">Activity Settings</h2>
        <div className="settings-grid">
          <div className="input-group">
            <label className="input-label">
              <input
                type="checkbox"
                checked={activityConfig.idle_detection_enabled}
                onChange={(e) =>
                  handleConfigChange({
                    idle_detection_enabled: e.target.checked,
                  })
                }
              />
              <span>Enable Idle Detection</span>
            </label>
          </div>

          <div className="input-group">
            <label className="input-label" htmlFor="idleThreshold">
              Idle Threshold (minutes)
            </label>
            <input
              id="idleThreshold"
              type="number"
              min="1"
              max="60"
              value={activityConfig.idle_threshold_minutes}
              onChange={(e) =>
                handleConfigChange({
                  idle_threshold_minutes: Math.max(
                    1,
                    Math.min(60, parseInt(e.target.value) || 1)
                  ),
                })
              }
              className="input-field"
              disabled={!activityConfig.idle_detection_enabled}
            />
          </div>

          <div className="input-group">
            <label className="input-label">
              <input
                type="checkbox"
                checked={activityConfig.require_idle_reason}
                onChange={(e) =>
                  handleConfigChange({ require_idle_reason: e.target.checked })
                }
                disabled={!activityConfig.idle_detection_enabled}
              />
              <span>Require Reason for Idle Time</span>
            </label>
          </div>
        </div>
      </div>

      {/* Idle Dialog */}
      {showIdleDialog && (
        <div className="idle-dialog">
          <div className="idle-dialog-content">
            <h2>Idle Time Detected</h2>
            <p>
              You were idle for some time. Would you like to keep this time?
            </p>

            <div className="idle-form">
              {activityConfig.require_idle_reason && (
                <textarea
                  value={idleReason}
                  onChange={(e) => setIdleReason(e.target.value)}
                  placeholder="Enter reason for idle time"
                  className="idle-reason-input"
                  required
                />
              )}

              <div className="idle-buttons">
                <button
                  onClick={() => handleIdleDecision(true)}
                  disabled={activityConfig.require_idle_reason && !idleReason}
                  className="keep-time-btn"
                >
                  Keep Time
                </button>
                <button
                  onClick={() => handleIdleDecision(false)}
                  className="discard-time-btn"
                >
                  Discard Time
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Screen Selection */}
      <div className="section">
        <h2 className="section-title">Available Screens</h2>
        <div className="screen-list">
          {screens.map((screen) => (
            <label key={screen.index} className="screen-item">
              <input
                type="checkbox"
                checked={selectedScreens.includes(screen.index)}
                onChange={() => toggleScreen(screen.index)}
              />
              <span className="screen-info">
                {screen.name} ({screen.width}x{screen.height})
              </span>
            </label>
          ))}
        </div>
        <div className="selected-count">
          {selectedScreens.length === 0
            ? "All screens will be captured"
            : `Selected: ${selectedScreens.length} screen(s)`}
        </div>
      </div>

      {/* Settings */}
      <div className="section">
        <h2 className="section-title">Settings</h2>
        <div className="settings-grid">
          <div className="input-group">
            <label className="input-label">Quality (1-100)</label>
            <input
              type="number"
              min="1"
              max="100"
              value={quality}
              onChange={(e) => setQuality(Number(e.target.value))}
              className="input-field"
            />
          </div>
          <div className="input-group">
            <label className="input-label">File Prefix</label>
            <input
              type="text"
              value={filePrefix}
              onChange={(e) => setFilePrefix(e.target.value)}
              className="input-field"
            />
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="section">
        <button onClick={takeScreenshot} className="capture-button">
          Capture Screenshot
        </button>
        <div className="status-text">{status}</div>
      </div>

      {/* Captured Screenshots */}
      {capturedScreenshots.length > 0 && (
        <div className="section">
          <h2 className="section-title">Captured Screenshots</h2>
          <div className="screenshots-grid">
            {capturedScreenshots.map((screenshot, index) => (
              <div key={index} className="screenshot-item">
                <img
                  src={`data:image/jpeg;base64,${screenshot.image_data}`}
                  alt={`Screen ${screenshot.screen_index}`}
                  className="screenshot-image"
                />
                <div className="screenshot-info">
                  Screen {screenshot.screen_index} ({screenshot.width}x
                  {screenshot.height})
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Time Tracking */}
      <div className="section">
        <h2 className="section-title">Time Tracking</h2>
        <div className="card">
          <div className="form-group">
            <label>Project:</label>
            <select
              value={selectedProject}
              onChange={(e) => setSelectedProject(e.target.value)}
              disabled={isTracking}
            >
              {projects.map((project) => (
                <option key={project.id} value={project.id}>
                  {project.name}
                </option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label>Task:</label>
            <select
              value={selectedTask}
              onChange={(e) => setSelectedTask(e.target.value)}
              disabled={isTracking}
            >
              {projects
                .find((p) => p.id === selectedProject)
                ?.tasks.map((task) => (
                  <option key={task.id} value={task.id}>
                    {task.name}
                  </option>
                ))}
            </select>
          </div>

          {activeEntry ? (
            <div className="active-entry">
              <div className="timer-display">
                <h2>{elapsedTime}</h2>
              </div>
              <div className="entry-details">
                <p>
                  <strong>Project: </strong>
                  {projects.find((p) => p.id === activeEntry.project_id)?.name}
                </p>
                <p>
                  <strong>Task: </strong>
                  {
                    projects
                      .find((p) => p.id === activeEntry.project_id)
                      ?.tasks.find((t) => t.id === activeEntry.task_id)?.name
                  }
                </p>
                <p>
                  <strong>Started: </strong>
                  {new Date(activeEntry.start_time).toLocaleString()}
                </p>
              </div>
            </div>
          ) : (
            <div className="timer-display">
              <h2>{elapsedTime}</h2>
            </div>
          )}

          <div className="button-group">
            {!isTracking ? (
              <button
                className="start-button"
                onClick={handleStartTracking}
                disabled={!selectedProject || !selectedTask}
              >
                Start Tracking
              </button>
            ) : (
              <button className="stop-button" onClick={handleStopTracking}>
                Stop Tracking
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
