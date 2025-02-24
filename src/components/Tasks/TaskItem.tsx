import { Task, SubTask } from "../../types";
import { formatDate } from "../../utils/formatDate";

interface TaskItemProps {
  task: Task;
  subtasks: SubTask[];
  isExpanded: boolean;
  isLoading: boolean;
  onToggle: () => void;
  onUpdateTask: (task: Task) => void;
  onUpdateSubtask: (subtask: SubTask) => void;
}

export function TaskItem({ task }: { task: any }) {
  return <div>{task.name}</div>;
}
