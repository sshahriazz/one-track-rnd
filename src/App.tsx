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

interface TimeEntry {
  id: string;
  project_id: string;
  task_id: string;
  start_time: string;
  end_time?: string;
  duration?: number;
}

function App() {
  const [screens, setScreens] = useState<ScreenInfo[]>([]);
  const [selectedScreens, setSelectedScreens] = useState<number[]>([]);
  const [quality, setQuality] = useState(85);
  const [filePrefix, setFilePrefix] = useState("screenshot");
  const [status, setStatus] = useState("");
  const [capturedScreenshots, setCapturedScreenshots] = useState<CapturedScreenshot[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<string>('');
  const [selectedTask, setSelectedTask] = useState<string>('');
  const [isTracking, setIsTracking] = useState(false);
  const [activeEntry, setActiveEntry] = useState<TimeEntry | null>(null);
  const [elapsedTime, setElapsedTime] = useState<string>('00:00:00');

  useEffect(() => {
    loadScreens();
    loadProjects();
    checkActiveEntry();
  }, []);

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
          `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
        );
      }, 1000);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [isTracking, activeEntry]);

  async function loadScreens() {
    try {
      const availableScreens = await invoke<ScreenInfo[]>("get_available_screens");
      setScreens(availableScreens);
    } catch (error) {
      setStatus("Failed to load screens: " + error);
    }
  }

  async function loadProjects() {
    try {
      const projectList = await invoke<Project[]>('get_all_projects');
      setProjects(projectList);
      if (projectList.length > 0) {
        setSelectedProject(projectList[0].id);
        if (projectList[0].tasks.length > 0) {
          setSelectedTask(projectList[0].tasks[0].id);
        }
      }
    } catch (error) {
      console.error('Error loading projects:', error);
    }
  }

  async function checkActiveEntry() {
    try {
      const entry = await invoke<TimeEntry | null>('get_active_entry');
      if (entry) {
        setIsTracking(true);
        setActiveEntry(entry);
        setSelectedProject(entry.project_id);
        setSelectedTask(entry.task_id);
      }
    } catch (error) {
      console.error('Error checking active entry:', error);
    }
  }

  async function takeScreenshot() {
    try {
      setStatus("Taking screenshot...");
      setCapturedScreenshots([]);
      
      await invoke("update_screenshot_config", {
        config: {
          mode: selectedScreens.length === 0 
            ? { All: null }
            : { Multiple: selectedScreens },
          quality,
          file_prefix: filePrefix
        }
      });

      const screenshots = await invoke<CapturedScreenshot[]>("take_screenshots");
      setCapturedScreenshots(screenshots);
      setStatus(`Successfully captured ${screenshots.length} screenshot(s)`);
    } catch (error) {
      setStatus("Failed to take screenshot: " + error);
    }
  }

  const toggleScreen = (index: number) => {
    setSelectedScreens(prev => 
      prev.includes(index)
        ? prev.filter(i => i !== index)
        : [...prev, index]
    );
  };

  const handleStartTracking = async () => {
    if (!selectedProject || !selectedTask) return;
    
    try {
      const entry = await invoke<TimeEntry>('start_time_tracking', {
        projectId: selectedProject,
        taskId: selectedTask,
      });
      setIsTracking(true);
      setActiveEntry(entry);
      setElapsedTime('00:00:00');
    } catch (error) {
      console.error('Error starting time tracking:', error);
    }
  };

  const handleStopTracking = async () => {
    try {
      await invoke<TimeEntry>('stop_time_tracking');
      setIsTracking(false);
      setActiveEntry(null);
      setElapsedTime('00:00:00');
    } catch (error) {
      console.error('Error stopping time tracking:', error);
    }
  };

  return (
    <div className="app-container">
      <h1 className="app-title">Screen Capture</h1>
      
      {/* Activity Tracker */}
      <ActivityTracker />
      
      {/* Screen Selection */}
      <div className="section">
        <h2 className="section-title">Available Screens</h2>
        <div className="screen-list">
          {screens.map(screen => (
            <label 
              key={screen.index}
              className="screen-item"
            >
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
                  Screen {screenshot.screen_index} ({screenshot.width}x{screenshot.height})
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
                  {projects.find(p => p.id === activeEntry.project_id)?.name}
                </p>
                <p>
                  <strong>Task: </strong>
                  {projects
                    .find(p => p.id === activeEntry.project_id)
                    ?.tasks.find(t => t.id === activeEntry.task_id)?.name}
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
              <button 
                className="stop-button"
                onClick={handleStopTracking}
              >
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
