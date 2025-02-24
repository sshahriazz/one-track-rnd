import { Button, cn } from "@heroui/react";
import { invoke } from "@tauri-apps/api/core";
import { motion, AnimatePresence } from "framer-motion";
import { TimerResponse } from "../../types";

interface ControlButtonProps {
  timerState: TimerResponse;
  disableBg?: boolean;
  enableAnimation?: boolean;
}

const buttonVariants = {
  initial: { scale: 0, opacity: 0, rotate: -180 },
  animate: {
    scale: 1,
    opacity: 1,
    rotate: 0,
    transition: {
      type: "spring",
      stiffness: 500,
      damping: 30,
      duration: 0.5,
    },
  },
  exit: {
    scale: 0,
    opacity: 0,
    rotate: 180,
    transition: {
      duration: 0.3,
    },
  },
};

const backgroundVariants = {
  initial: {
    scale: 0.8,
    opacity: 0,
    x: -50,
  },
  animate: {
    scale: 1,
    opacity: 1,
    x: 0,
    transition: {
      type: "spring",
      stiffness: 400,
      damping: 25,
      duration: 0.5,
    },
  },
  exit: {
    scale: 0.8,
    opacity: 0,
    x: 50,
    transition: {
      duration: 0.3,
    },
  },
};

function ControlButton({
  timerState,
  disableBg = false,
  enableAnimation = true,
}: ControlButtonProps) {
  const handleTimerControl = async (
    command: "Start" | "Pause" | "Resume" | "Stop"
  ) => {
    try {
      await invoke("control_timer", { command });
    } catch (error) {
      console.error("Failed to control timer:", error);
    }
  };

  const getButtonConfig = () => {
    if (!timerState.running && timerState.elapsed_seconds === 0) {
      return {
        command: "Start",
        label: "Start",
        color: "success",
        textColor: "text-success-600",
        borderColor: "border-success-600",
      };
    }
    if (timerState.running) {
      return {
        command: "Pause",
        label: "Pause",
        color: "warning",
        textColor: "text-warning-600",
        borderColor: "border-warning-600",
      };
    }
    return {
      command: "Resume",
      label: "Resume",
      color: "primary",
      textColor: "text-primary-600",
      borderColor: "border-primary-600",
    };
  };

  const buttonConfig = getButtonConfig();

  return (
    <motion.div
      className="relative flex gap-4"
      layout
      transition={{ type: "spring", stiffness: 300, damping: 25 }}
    >
      <motion.div
        className="relative"
        layout
        initial="initial"
        animate="animate"
        exit="exit"
        transition={{ type: "spring", stiffness: 500, damping: 30 }}
      >
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
        />
        <motion.div
          className="relative size-28 flex items-center justify-center"
          layout
        >
          <motion.div
            initial="initial"
            animate="animate"
            exit="exit"
            variants={buttonVariants}
          >
            <Button
              onPress={() => handleTimerControl(buttonConfig.command as any)}
              variant="shadow"
              color={buttonConfig.color as any}
              className={cn(
                `size-20 ${buttonConfig.textColor} border-2 ${buttonConfig.borderColor} font-semibold bg-white rounded-[100%] flex items-center justify-center`
              )}
            >
              {buttonConfig.label}
            </Button>
          </motion.div>
        </motion.div>
      </motion.div>

      <AnimatePresence mode="popLayout">
        {!timerState.running && timerState.elapsed_seconds > 0 && (
          <motion.div
            className="relative"
            layout
            initial="initial"
            animate="animate"
            exit="exit"
            variants={backgroundVariants}
          >
            <motion.div
              className="absolute inset-0 size-28 flex items-center justify-center"
              animate={{
                scale: 1,
                borderRadius: "1032.353px",
                background: "rgba(239, 68, 68, 0.20)",
              }}
            />
            <motion.div
              className="absolute inset-0 size-24 m-auto flex items-center justify-center"
              animate={{
                scale: 1,
                borderRadius: "1032.353px",
                background: "rgba(239, 68, 68, 0.30)",
              }}
            />
            <motion.div
              className="relative size-28 flex items-center justify-center"
              layout
            >
              <motion.div
                initial="initial"
                animate="animate"
                exit="exit"
                variants={buttonVariants}
              >
                <Button
                  onPress={() => handleTimerControl("Stop")}
                  variant="shadow"
                  color="danger"
                  className={cn(
                    "size-20 text-danger-600 border-2 border-danger-600 font-semibold bg-white rounded-[100%] flex items-center justify-center"
                  )}
                >
                  Stop
                </Button>
              </motion.div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}

export default ControlButton;
