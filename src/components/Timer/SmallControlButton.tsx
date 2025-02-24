import { Button } from "@heroui/react";
import { motion, AnimatePresence } from "framer-motion";
import React, { useState } from "react";

function SmallControlButton() {
  const [isPlaying, setIsPlaying] = useState(false);

  const iconVariants = {
    play: {
      pathLength: 1,
      pathOffset: 0,
      opacity: 1,
    },
    pause: {
      pathLength: 1,
      pathOffset: 0,
      opacity: 1,
    },
    exit: {
      pathLength: 0,
      pathOffset: 1,
      opacity: 0,
    }
  };

  return (
    <Button 
      variant="light" 
      isIconOnly 
      size="sm"
      onClick={() => setIsPlaying(!isPlaying)}
    >
      <motion.div
        whileTap={{ scale: 0.95 }}
        className="relative"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="17"
          viewBox="0 0 16 17"
          fill="none"
        >
          <motion.path
            d="M8.00016 15.1667C11.6821 15.1667 14.6668 12.1819 14.6668 8.50004C14.6668 4.81814 11.6821 1.83337 8.00016 1.83337C4.31826 1.83337 1.3335 4.81814 1.3335 8.50004C1.3335 12.1819 4.31826 15.1667 8.00016 15.1667Z"
            stroke="#0246A7"
            strokeWidth="1.2"
            animate={{
              rotate: isPlaying ? [0, 180, 360] : 0,
            }}
            transition={{
              rotate: { duration: 0.4, ease: "easeInOut" },
            }}
          />
          <AnimatePresence mode="wait">
            {!isPlaying ? (
              <motion.path
                key="play"
                d="M10.302 7.88196L7.41683 6.26145C6.9341 5.99032 6.3335 6.33293 6.3335 6.87943V10.1205C6.3335 10.667 6.9341 11.0096 7.41683 10.7385L10.302 9.11796C10.7884 8.84469 10.7884 8.15523 10.302 7.88196Z"
                fill="#0246A7"
                initial="exit"
                animate="play"
                exit="exit"
                variants={iconVariants}
                transition={{ duration: 0.2, ease: "easeInOut" }}
              />
            ) : (
              <motion.path
                key="pause"
                d="M6.5 6.5h1.2v4h-1.2v-4zm2.8 0h1.2v4h-1.2v-4z"
                fill="#0246A7"
                initial="exit"
                animate="pause"
                exit="exit"
                variants={iconVariants}
                transition={{ duration: 0.2, ease: "easeInOut" }}
              />
            )}
          </AnimatePresence>
        </svg>
      </motion.div>
    </Button>
  );
}

export default SmallControlButton;
