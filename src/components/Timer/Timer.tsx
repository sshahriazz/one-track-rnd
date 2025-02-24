import { Button } from "@heroui/react";
import { TimerResponse } from "../../types";
import { formatTime } from "../../utils/formatTime";
import ControlButton from "./ControlButton";

interface TimerProps {
  timerState: TimerResponse;
}

export function Timer() {
  return (
    <div className="flex flex-col items-center py-8 justify-center bg-white">
      {/* Timer Display */}
      <div className="text-2xl font-bold text-primary-700 mb-2">00:00:00</div>

      {/* Title and Subtitle */}
      <div className="text-center mb-4">
        <h1 className="text-lg font-bold text-gray-800">Create Time Tracker</h1>
        <h2 className="text-xs text-gray-500">Tracker Running</h2>
      </div>

      <ControlButton />
    </div>
  );
}
