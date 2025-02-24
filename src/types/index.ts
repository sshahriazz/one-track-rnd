export interface TimerResponse {
  elapsed_seconds: number;
  running: boolean;
  start_time: string | null;
  end_time: string | null;
}

export interface Project {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
  version: number;
}

export interface Section {
  id: string;
  name: string;
  project_id: string;
  created_at: string;
  updated_at: string;
  version: number;
}

export interface Task {
  id: string;
  name: string;
  section_id: string;
  created_at: string;
  updated_at: string;
  version: number;
  description?: string;
  status: 'todo' | 'in_progress' | 'completed';
}

export interface SubTask {
  id: string;
  name: string;
  task_id: string;
  created_at: string;
  updated_at: string;
  version: number;
  completed: boolean;
}

export interface LoadingState {
  projects: boolean;
  sections: Set<string>;
  tasks: Set<string>;
  subtasks: Set<string>;
}
