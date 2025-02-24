import { Button, cn } from "@heroui/react";
import { invoke } from "@tauri-apps/api/core";
import { motion } from "framer-motion";
import { useState } from "react";

interface ControlButtonProps {
  disableBg?: boolean;
  enableAnimation?: boolean;
}

function ControlButton({
  disableBg = false,
  enableAnimation = true,
}: ControlButtonProps) {
  const [isRunning, setIsRunning] = useState(false);
  return (
    <div className="relative">
      <motion.div
        className={
          disableBg
            ? ""
            : "absolute inset-0 size-28 flex items-center justify-center"
        }
        animate={
          enableAnimation
            ? {
                scale: [1, 1.1, 1, 1.15, 1],
                borderRadius: ["1032.353px", "1032.353px"],
                background: [
                  "rgba(206, 232, 217, 0.10)",
                  "rgba(206, 232, 217, 0.20)",
                ],
              }
            : {
                scale: 1,
                borderRadius: "1032.353px",
                background: "rgba(206, 232, 217, 0.20)",
              }
        }
        transition={
          enableAnimation
            ? {
                duration: 1.4,
                times: [0, 0.14, 0.28, 0.42, 0.7],
                ease: "easeInOut",
                repeat: Infinity,
              }
            : {}
        }
        style={disableBg ? {} : {}}
      />
      <motion.div
        className={
          disableBg
            ? ""
            : "absolute inset-0 size-24 m-auto flex items-center justify-center"
        }
        animate={
          enableAnimation
            ? {
                scale: [1, 1.15, 1, 1.2, 1],
                borderRadius: ["876.526px", "876.526px"],
                background: [
                  "rgba(206, 232, 217, 0.15)",
                  "rgba(206, 232, 217, 0.30)",
                ],
              }
            : {
                scale: 1,
                borderRadius: "1032.353px",
                background: "rgba(206, 232, 217, 0.20)",
              }
        }
        transition={
          enableAnimation
            ? {
                duration: 1.4,
                times: [0, 0.14, 0.28, 0.42, 0.7],
                ease: "easeInOut",
                repeat: Infinity,
              }
            : {}
        }
        style={disableBg ? {} : {}}
      />
      <div className="relative size-28 flex items-center justify-center">
        <Button
          onPress={async () => {
            if (!isRunning) {
              await invoke("control_timer", { command: "Start" });
              console.log("Timer started");
              setIsRunning(true);
            } else {
              await invoke("control_timer", { command: "Stop" });
              console.log("Timer stopped");
              setIsRunning(false);
            }
          }}
          variant="shadow"
          color="success"
          className={cn(
            "size-20 text-success-600 border-2 border-success-600 font-semibold bg-white rounded-[100%] flex items-center justify-center"
          )}
        >
          {isRunning ? "Stop" : "Start"}
        </Button>
      </div>
    </div>
  );
}

export default ControlButton;
