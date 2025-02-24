import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";

import { Timer } from "./components/Timer/Timer";
import { ProjectList } from "./components/Projects/ProjectList";
import { TaskList } from "./components/Tasks/TaskList";
import {
  Project,
  Section,
  Task,
  SubTask,
  TimerResponse,
  LoadingState,
} from "./types";

function App() {
  // Navigation State
  const [currentView, setCurrentView] = useState<
    "projects" | "sections" | "tasks"
  >("projects");
  const [selectedProject, setSelectedProject] = useState<string | null>(null);
  // Timer State
  const [timerState, setTimerState] = useState<TimerResponse>({
    elapsed_seconds: 0,
    running: false,
    start_time: null,
    end_time: null,
  });

  // Project and Section State
  const [projects, setProjects] = useState<Project[]>([]);
  const [expandedProjects, setExpandedProjects] = useState<Set<string>>(
    new Set()
  );
  const [projectSections, setProjectSections] = useState<
    Record<string, Section[]>
  >({});

  // Task and Subtask State
  const [selectedSection, setSelectedSection] = useState<string | null>(null);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [expandedTasks, setExpandedTasks] = useState<Set<string>>(new Set());
  const [taskSubtasks, setTaskSubtasks] = useState<Record<string, SubTask[]>>(
    {}
  );

  // Loading and Error State
  const [loading, setLoading] = useState<LoadingState>({
    projects: true,
    sections: new Set<string>(),
    tasks: new Set<string>(),
    subtasks: new Set<string>(),
  });
  const [error, setError] = useState<string | null>(null);

  // Initialize app and set up timer updates
  useEffect(() => {
    const initializeApp = async () => {
      try {
        setLoading((prev) => ({ ...prev, projects: true }));
        const fetchedProjects = await invoke<Project[]>("get_projects");
        setProjects(fetchedProjects);
        await invoke("start_timer_updates");
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to initialize app"
        );
        console.error(err);
      } finally {
        setLoading((prev) => ({ ...prev, projects: false }));
      }
    };

    const unsubscribePromise = listen<TimerResponse>(
      "timer-update",
      (event) => {
        setTimerState(event.payload);
      }
    );

    initializeApp();

    return () => {
      unsubscribePromise.then((fn) => fn());
    };
  }, []);

  // Fetch tasks when section is selected
  useEffect(() => {
    if (selectedSection) {
      const fetchTasks = async () => {
        try {
          setLoading((prev) => ({
            ...prev,
            tasks: new Set([...prev.tasks, selectedSection]),
          }));
          const fetchedTasks = await invoke<Task[]>("get_tasks_by_section_id", {
            sectionId: selectedSection,
          });
          setTasks(fetchedTasks);
        } catch (err) {
          setError(
            err instanceof Error ? err.message : "Failed to fetch tasks"
          );
          console.error(err);
        } finally {
          setLoading((prev) => {
            const newTasks = new Set(prev.tasks);
            newTasks.delete(selectedSection);
            return { ...prev, tasks: newTasks };
          });
        }
      };

      fetchTasks();
    } else {
      setTasks([]);
      setTaskSubtasks({});
      setExpandedTasks(new Set());
    }
  }, [selectedSection]);

  // Project and Section handlers
  // Project handlers
  const handleToggleProject = async (projectId: string) => {
    const newExpandedProjects = new Set(expandedProjects);

    if (expandedProjects.has(projectId)) {
      newExpandedProjects.delete(projectId);
      if (selectedSection) {
        const sectionExists = projectSections[projectId]?.some(
          (s) => s.id === selectedSection
        );
        if (sectionExists) {
          setSelectedSection(null);
        }
      }
    } else {
      newExpandedProjects.add(projectId);
      if (!projectSections[projectId]) {
        try {
          setLoading((prev) => ({
            ...prev,
            sections: new Set([...prev.sections, projectId]),
          }));
          const sections = await invoke<Section[]>(
            "get_sections_by_project_id",
            { projectId }
          );
          setProjectSections((prev) => ({ ...prev, [projectId]: sections }));
        } catch (error) {
          setError(
            error instanceof Error ? error.message : "Failed to fetch sections"
          );
          console.error("Error fetching sections:", error);
          newExpandedProjects.delete(projectId);
        } finally {
          setLoading((prev) => {
            const newSections = new Set(prev.sections);
            newSections.delete(projectId);
            return { ...prev, sections: newSections };
          });
        }
      }
    }

    setExpandedProjects(newExpandedProjects);
  };

  const handleSelectProject = async (projectId: string) => {
    setSelectedProject(projectId);
    setCurrentView("sections");

    // Load sections for the selected project if not already loaded
    if (!projectSections[projectId]) {
      try {
        setLoading((prev) => ({
          ...prev,
          sections: new Set([...prev.sections, projectId]),
        }));
        const sections = await invoke<Section[]>("get_project_sections", {
          projectId,
        });
        setProjectSections((prev) => ({
          ...prev,
          [projectId]: sections,
        }));
      } catch (err) {
        setError(
          err instanceof Error ? err.message : "Failed to load sections"
        );
      } finally {
        setLoading((prev) => {
          const newSections = new Set(prev.sections);
          newSections.delete(projectId);
          return { ...prev, sections: newSections };
        });
      }
    }
  };

  const handleSelectSection = async (sectionId: string) => {
    setSelectedSection(sectionId);
    setCurrentView("tasks");

    try {
      setLoading((prev) => ({
        ...prev,
        tasks: new Set([...prev.tasks, sectionId]),
      }));
      const tasks = await invoke<Task[]>("get_section_tasks", { sectionId });
      setTasks(tasks);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load tasks");
    } finally {
      setLoading((prev) => {
        const newTasks = new Set(prev.tasks);
        newTasks.delete(sectionId);
        return { ...prev, tasks: newTasks };
      });
    }
  };

  const handleBackToProjects = () => {
    setCurrentView("projects");
    setSelectedProject(null);
    setSelectedSection(null);
    setTasks([]);
  };

  const handleBackToSections = () => {
    setCurrentView("sections");
    setSelectedSection(null);
    setTasks([]);
  };

  // Task and Subtask handlers
  // Task handlers
  const handleToggleTask = async (taskId: string) => {
    const newExpandedTasks = new Set(expandedTasks);

    if (newExpandedTasks.has(taskId)) {
      newExpandedTasks.delete(taskId);
    } else {
      newExpandedTasks.add(taskId);
      if (!taskSubtasks[taskId]) {
        try {
          setLoading((prev) => ({
            ...prev,
            subtasks: new Set([...prev.subtasks, taskId]),
          }));
          const subtasks = await invoke<SubTask[]>("get_sub_tasks_by_task_id", {
            taskId,
          });
          setTaskSubtasks((prev) => ({ ...prev, [taskId]: subtasks }));
        } catch (error) {
          setError(
            error instanceof Error ? error.message : "Failed to fetch subtasks"
          );
          console.error("Error fetching subtasks:", error);
          newExpandedTasks.delete(taskId);
        } finally {
          setLoading((prev) => {
            const newSubtasks = new Set(prev.subtasks);
            newSubtasks.delete(taskId);
            return { ...prev, subtasks: newSubtasks };
          });
        }
      }
    }

    setExpandedTasks(newExpandedTasks);
  };

  const handleUpdateTask = async (task: Task) => {
    // TODO: Implement task update IPC
    console.log("Update task:", task);
  };

  const handleUpdateSubtask = async (subtask: SubTask) => {
    // TODO: Implement subtask update IPC
    console.log("Update subtask:", subtask);
  };

  // Effect for initial project loading
  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const projects = await invoke<Project[]>("get_projects");
        setProjects(projects);
      } catch (error) {
        setError(
          error instanceof Error ? error.message : "Failed to fetch projects"
        );
        console.error("Error fetching projects:", error);
      } finally {
        setLoading((prev) => ({ ...prev, projects: false }));
      }
    };

    fetchProjects();
  }, []);

  // Timer handlers
  const handleStartTimer = async () => {
    try {
      await invoke("control_timer", { command: "Start" });
    } catch (error) {
      setError(
        error instanceof Error ? error.message : "Failed to start timer"
      );
      console.error("Error starting timer:", error);
    }
  };

  const handlePauseTimer = async () => {
    try {
      await invoke("control_timer", { command: "Pause" });
    } catch (error) {
      setError(
        error instanceof Error ? error.message : "Failed to pause timer"
      );
      console.error("Error pausing timer:", error);
    }
  };

  const handleResumeTimer = async () => {
    try {
      await invoke("control_timer", { command: "Resume" });
    } catch (error) {
      setError(
        error instanceof Error ? error.message : "Failed to resume timer"
      );
      console.error("Error resuming timer:", error);
    }
  };

  const handleStopTimer = async () => {
    try {
      await invoke("control_timer", { command: "Stop" });
    } catch (error) {
      setError(error instanceof Error ? error.message : "Failed to stop timer");
      console.error("Error stopping timer:", error);
    }
  };

  const handleAddTime = async (seconds: number) => {
    try {
      await invoke("control_timer", {
        command: { AddTime: seconds },
      });
    } catch (error) {
      setError(error instanceof Error ? error.message : "Failed to add time");
      console.error("Error adding time:", error);
    }
  };

  return (
    <div className="app-container">
      <div className="main-content">
        <div className="navigation-panel">
          {currentView === "projects" && (
            <ProjectList
              projects={projects}
              loading={loading}
              onSelectProject={handleSelectProject}
            />
          )}

          {currentView === "sections" && selectedProject && (
            <div className="sections-view">
              <button className="back-button" onClick={handleBackToProjects}>
                ← Back to Projects
              </button>
              <div className="sections-list">
                {projectSections[selectedProject]?.map((section) => (
                  <div
                    key={section.id}
                    className="section-item"
                    onClick={() => handleSelectSection(section.id)}
                  >
                    <h3>{section.name}</h3>
                    <span className="version">v{section.version}</span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {currentView === "tasks" && (
            <div className="tasks-view">
              <button className="back-button" onClick={handleBackToSections}>
                ← Back to Sections
              </button>
              <TaskList
                tasks={tasks}
                subtasks={taskSubtasks}
                loading={loading}
                onUpdateSubtask={handleUpdateSubtask}
                expandedTasks={expandedTasks}
                onToggleTask={handleToggleTask}
                onUpdateTask={handleUpdateTask}
              />
            </div>
          )}
        </div>

        <div className="timer-panel">
          <div className="timer-header">
            <h1>Timer</h1>
            <div className="breadcrumbs">
              {selectedProject && (
                <span>
                  {projects.find((p) => p.id === selectedProject)?.name}
                </span>
              )}
              {selectedSection &&
                projectSections[selectedProject!]?.find(
                  (s) => s.id === selectedSection
                ) && (
                  <span>
                    {" "}
                    →{" "}
                    {
                      projectSections[selectedProject!].find(
                        (s) => s.id === selectedSection
                      )?.name
                    }
                  </span>
                )}
            </div>
          </div>

          <Timer
            timerState={timerState}
            onStart={handleStartTimer}
            onStop={handleStopTimer}
            onPause={handlePauseTimer}
            onResume={handleResumeTimer}
            onAddTime={handleAddTime}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
