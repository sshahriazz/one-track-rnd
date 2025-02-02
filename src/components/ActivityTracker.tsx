import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./ActivityTracker.css";

interface ActivityConfig {
  track_keyboard: boolean;
  track_mouse: boolean;
  is_tracking: boolean;
}

interface ActivityStatus {
  keyboard_active: boolean;
  mouse_active: boolean;
}

export function ActivityTracker() {
  const [config, setConfig] = useState<ActivityConfig>({
    track_keyboard: true,
    track_mouse: true,
    is_tracking: false,
  });
  const [status, setStatus] = useState<ActivityStatus>({
    keyboard_active: false,
    mouse_active: false,
  });
  const [error, setError] = useState<string>("");
  const [activityHistory, setActivityHistory] = useState<{
    time: number;
    keyboard: boolean;
    mouse: boolean;
  }[]>([]);
  
  const pollingInterval = useRef<number | null>(null);

  // Load initial config
  useEffect(() => {
    loadConfig();
    return () => stopPolling();
  }, []);

  // Start/stop polling based on tracking state
  useEffect(() => {
    if (config.is_tracking) {
      startPolling();
    } else {
      stopPolling();
      // Clear status when tracking stops
      setStatus({ keyboard_active: false, mouse_active: false });
    }
  }, [config.is_tracking]);

  function startPolling() {
    if (!pollingInterval.current) {
      pollingInterval.current = window.setInterval(pollActivityStatus, 1000);
    }
  }

  function stopPolling() {
    if (pollingInterval.current) {
      clearInterval(pollingInterval.current);
      pollingInterval.current = null;
    }
  }

  async function loadConfig() {
    try {
      const savedConfig = await invoke<ActivityConfig>("get_activity_config");
      setConfig(savedConfig);
      setError("");
    } catch (err) {
      setError("Failed to load config: " + err);
    }
  }

  async function updateConfig(newConfig: ActivityConfig) {
    try {
      await invoke("update_activity_config", { config: newConfig });
      setConfig(newConfig);
      setError("");
    } catch (err) {
      setError("Failed to update config: " + err);
    }
  }

  async function startTracking() {
    try {
      await invoke("start_activity_tracking");
      setConfig(prev => ({ ...prev, is_tracking: true }));
      setError("");
    } catch (err) {
      setError("Failed to start tracking: " + err);
    }
  }

  async function stopTracking() {
    try {
      await invoke("stop_activity_tracking");
      setConfig(prev => ({ ...prev, is_tracking: false }));
      setError("");
    } catch (err) {
      setError("Failed to stop tracking: " + err);
    }
  }

  async function pollActivityStatus() {
    try {
      const currentStatus = await invoke<ActivityStatus>("get_activity_status");
      setStatus(currentStatus);
      
      // If there's no activity, clear the history
      if (!currentStatus.keyboard_active && !currentStatus.mouse_active) {
        setActivityHistory([]);
      } else {
        // Only add to history if there's activity
        const now = Date.now();
        setActivityHistory(prev => [
          ...prev.slice(-9), // Keep last 10 seconds instead of 60
          {
            time: now,
            keyboard: currentStatus.keyboard_active,
            mouse: currentStatus.mouse_active,
          },
        ]);
      }
      
      setError("");
    } catch (err) {
      setError("Failed to get activity status: " + err);
    }
  }

  // Calculate activity percentage for the last 10 seconds
  const getActivityPercentage = (type: 'keyboard' | 'mouse') => {
    if (activityHistory.length === 0) return 0;
    const activeCount = activityHistory.filter(h => h[type]).length;
    return Math.round((activeCount / Math.max(10, activityHistory.length)) * 100);
  };

  return (
    <div className="section">
      <h2 className="section-title">Activity Tracker</h2>
      
      {/* Configuration */}
      <div className="config-group">
        <div className="config-row">
          <label className="config-item">
            <input
              type="checkbox"
              checked={config.track_keyboard}
              onChange={(e) =>
                updateConfig({ ...config, track_keyboard: e.target.checked })
              }
              disabled={config.is_tracking}
            />
            Track Keyboard
          </label>
          <label className="config-item">
            <input
              type="checkbox"
              checked={config.track_mouse}
              onChange={(e) =>
                updateConfig({ ...config, track_mouse: e.target.checked })
              }
              disabled={config.is_tracking}
            />
            Track Mouse/Trackpad
          </label>
        </div>

        <div className="control-buttons">
          {!config.is_tracking ? (
            <button 
              className="primary-button"
              onClick={startTracking}
              disabled={!config.track_keyboard && !config.track_mouse}
            >
              Start Tracking
            </button>
          ) : (
            <button 
              className="secondary-button"
              onClick={stopTracking}
            >
              Stop Tracking
            </button>
          )}
        </div>
      </div>

      {/* Current Status */}
      {config.is_tracking && (
        <>
          <div className="status-group">
            <div className="status-indicators">
              <div className={`status-indicator ${status.keyboard_active ? 'active' : ''}`}>
                <span className="indicator-label">Keyboard</span>
                <span className="indicator-dot"></span>
              </div>
              <div className={`status-indicator ${status.mouse_active ? 'active' : ''}`}>
                <span className="indicator-label">Mouse</span>
                <span className="indicator-dot"></span>
              </div>
            </div>
          </div>

          {/* Activity Stats */}
          <div className="stats-group">
            <div className="activity-bars">
              <div className="activity-bar">
                <label>Keyboard</label>
                <div className="bar-container">
                  <div 
                    className="bar-fill" 
                    style={{ width: `${getActivityPercentage('keyboard')}%` }}
                  />
                  <span className="bar-label">{getActivityPercentage('keyboard')}%</span>
                </div>
              </div>
              <div className="activity-bar">
                <label>Mouse</label>
                <div className="bar-container">
                  <div 
                    className="bar-fill" 
                    style={{ width: `${getActivityPercentage('mouse')}%` }}
                  />
                  <span className="bar-label">{getActivityPercentage('mouse')}%</span>
                </div>
              </div>
            </div>
          </div>
        </>
      )}

      {error && <div className="error-message">{error}</div>}
    </div>
  );
}
