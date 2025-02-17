import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

interface TimerResponse {
  elapsed_seconds: number;
  running: boolean;
  start_time: string | null;
  end_time: string | null;
}

function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const ms = Math.floor((seconds % 1) * 100);

  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}:${secs.toString().padStart(2, "0")}:${ms
    .toString()
    .padStart(2, "0")}`;
}

function App() {
  const [timerState, setTimerState] = useState<TimerResponse>({
    elapsed_seconds: 0,
    running: false,
    start_time: null,
    end_time: null,
  });

  useEffect(() => {
    // Start the timer updates when component mounts
    invoke("start_timer_updates").catch(console.error);

    // Listen to timer updates from the backend
    const unsubscribe = listen<TimerResponse>("timer-update", (event) => {
      setTimerState(event.payload);
    });

    return () => {
      // Cleanup listener when component unmounts
      unsubscribe.then((fn) => fn());
    };
  }, []);

  const startTimer = async () => {
    try {
      await invoke("control_timer", { command: "Start" });
    } catch (error) {
      console.error("Error starting timer:", error);
    }
  };

  const pauseTimer = async () => {
    try {
      await invoke("control_timer", { command: "Pause" });
    } catch (error) {
      console.error("Error pausing timer:", error);
    }
  };

  const resumeTimer = async () => {
    try {
      await invoke("control_timer", { command: "Resume" });
    } catch (error) {
      console.error("Error resuming timer:", error);
    }
  };

  const stopTimer = async () => {
    try {
      await invoke("control_timer", { command: "Stop" });
    } catch (error) {
      console.error("Error stopping timer:", error);
    }
  };

  const addTime = async (seconds: number) => {
    try {
      await invoke("control_timer", {
        command: { AddTime: seconds }
      });
    } catch (error) {
      console.error("Error adding time:", error);
    }
  };

  return (
    <div className="container">
      <div className="timer-container">
        <h1>Timer</h1>
        <div className="timer-display">
          <h2>{formatTime(timerState.elapsed_seconds)}</h2>
          {timerState.start_time && (
            <p>Started: {new Date(timerState.start_time).toLocaleString()}</p>
          )}
          {timerState.end_time && (
            <p>Ended: {new Date(timerState.end_time).toLocaleString()}</p>
          )}
        </div>
        <div className="controls">
          {/* Show Start only when timer is completely stopped */}
          <button 
            onClick={startTimer} 
            disabled={timerState.running || timerState.elapsed_seconds > 0}
          >
            Start New
          </button>

          {/* Show Resume only when timer is paused (not running but has elapsed time) */}
          <button 
            onClick={resumeTimer} 
            disabled={timerState.running || timerState.elapsed_seconds === 0}
          >
            Resume
          </button>

          {/* Show Pause only when timer is running */}
          <button 
            onClick={pauseTimer} 
            disabled={!timerState.running}
          >
            Pause
          </button>

          {/* Stop is available when timer is either running or has elapsed time */}
          <button 
            onClick={stopTimer} 
            disabled={!timerState.running && timerState.elapsed_seconds === 0}
          >
            Stop
          </button>
        </div>

        <div className="time-controls">
          {/* Time adjustment controls only available when timer is not running */}
          <button 
            onClick={() => addTime(5)} 
            disabled={timerState.running}
            title="Add 5 seconds"
          >
            +5s
          </button>
          <button 
            onClick={() => addTime(30)} 
            disabled={timerState.running}
            title="Add 30 seconds"
          >
            +30s
          </button>
          <button 
            onClick={() => addTime(60)} 
            disabled={timerState.running}
            title="Add 1 minute"
          >
            +1m
          </button>
          <button 
            onClick={() => addTime(300)} 
            disabled={timerState.running}
            title="Add 5 minutes"
          >
            +5m
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
