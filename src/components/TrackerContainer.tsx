import { Timer } from "./Timer/Timer";
import { ProjectList } from "./Projects/ProjectList";
import { useState, useEffect } from "react";

import {
  Dropdown,
  DropdownTrigger,
  DropdownMenu,
  DropdownSection,
  DropdownItem,
  Button,
  Input,
} from "@heroui/react";
import { TaskList } from "./Tasks/TaskList";

const TrackerContainer = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [isScrolled, setIsScrolled] = useState(false);

  // Handle header shadow on scroll
  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 0);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <div className="relative flex flex-col sm:grid sm:grid-cols-[320px_1fr] h-screen overflow-hidden bg-gray-50">
      {/* Mobile Menu Button */}
      <button
        onClick={() => setIsSidebarOpen(!isSidebarOpen)}
        className="sm:hidden fixed top-4 left-4 z-50 p-2.5 rounded-xl bg-white/80 backdrop-blur-sm shadow-sm border border-gray-200/60 hover:bg-white/90 active:bg-white transition-colors"
        aria-label="Toggle menu"
      >
        <svg
          className="w-5 h-5 text-gray-700"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d={
              isSidebarOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"
            }
          />
        </svg>
      </button>

      {/* Left Section - Timer and Projects */}
      <div
        className={`
          fixed sm:relative w-full sm:w-auto h-full bg-white z-40
          transform transition-all duration-300 ease-in-out
          ${
            isSidebarOpen
              ? "translate-x-0"
              : "-translate-x-full sm:translate-x-0"
          }
          border-r border-gray-200/80
        `}
      >
        <div className="border-b border-gray-200/80 bg-white">
          <Timer />
        </div>
        <div className="flex-1 bg-white/50 backdrop-blur-sm">
          <ProjectList />
        </div>
      </div>

      {/* Right Section - Tasks */}
      <div className="flex-1 min-w-0 relative flex flex-col overflow-hidden">
        {/* Header */}

        <div className="px-4 sm:px-6 py-4">
          <div className="flex flex-col justify-start gap-3">
            <h2 className="text-xl font-semibold text-gray-900">
              Business Development
            </h2>
            <div className="flex flex-row items-center justify-between gap-3 sm:gap-6">
              <Dropdown>
                <DropdownTrigger>
                  <Button variant="solid" color="primary">
                    Select Section
                  </Button>
                </DropdownTrigger>
                <DropdownMenu aria-label="Static Actions">
                  <DropdownItem key="new">Inprogress</DropdownItem>
                  <DropdownItem key="copy">Todo</DropdownItem>
                  <DropdownItem key="edit">InReview</DropdownItem>
                </DropdownMenu>
              </Dropdown>

              <Input
                type="text"
                variant="bordered"
                color="primary"
                className="w-50"
                placeholder="Search task"
                endContent={
                  <svg
                    className="w-5 h-5 text-gray-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                  </svg>
                }
              />
            </div>
          </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-auto p-4 sm:p-6">
          <TaskList />
        </div>
      </div>

      {/* Overlay for mobile when sidebar is open */}
      {isSidebarOpen && (
        <div
          className="
            fixed inset-0 bg-gray-900/20 backdrop-blur-sm z-30 sm:hidden
            transition-opacity duration-300
          "
          onClick={() => setIsSidebarOpen(false)}
        />
      )}
    </div>
  );
};

export default TrackerContainer;
