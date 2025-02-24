import * as React from "react";

// 1. import `HeroUIProvider` component
import { Button, HeroUIProvider } from "@heroui/react";
import { Timer } from "./components/Timer/Timer";
import TrackerContainer from "./components/TrackerContainer";

export default function App() {
  // 2. Wrap HeroUIProvider at the root of your app
  return (
    <HeroUIProvider>
      <TrackerContainer />
    </HeroUIProvider>
  );
}
