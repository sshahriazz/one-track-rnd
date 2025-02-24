import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { TimerResponse } from "../../types";
import { formatTime } from "../../utils/formatTime";
import ControlButton from "./ControlButton";

export function Timer() {
  const [timerState, setTimerState] = useState<TimerResponse>({
    elapsed_seconds: 0,
    running: false,
    start_time: null,
    end_time: null,
  });

  useEffect(() => {
    // Start the timer update service
    invoke("start_timer_updates").catch(console.error);

    // Listen for timer updates
    const unlisten = listen<TimerResponse>("timer-update", (event) => {
      setTimerState(event.payload);
    });

    return () => {
      unlisten.then((unlistenFn) => unlistenFn());
    };
  }, []);

  return (
    <div className="flex flex-col items-center py-8 justify-center bg-white">
      {/* Timer Display */}
      <div className="text-2xl font-bold text-primary-700 mb-2">
        {formatTime(timerState.elapsed_seconds)}
      </div>

      {/* Title and Subtitle */}
      <div className="text-center mb-4">
        <h1 className="text-lg font-bold text-gray-800">Create Time Tracker</h1>
        <h2 className="text-xs text-gray-500">
          {timerState.running ? "Tracker Running" : "Tracker Stopped"}
        </h2>
      </div>

      <ControlButton timerState={timerState} />
    </div>
  );
}
