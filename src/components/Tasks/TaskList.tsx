import { Listbox, ListboxItem } from "@heroui/react";
import { Task, SubTask, LoadingState } from "../../types";
import { TaskItem } from "./TaskItem";
import SmallControlButton from "../Timer/SmallControlButton";

const tasks = [
  {
    id: "1",
    name: "Task 1",
    project_id: "1",
    section_id: "1",
    created_at: "2020-01-01T00:00:00.000Z",
    updated_at: "2020-01-01T00:00:00.000Z",
    version: 1,
  },
  {
    id: "2",
    name: "Task 2",
    project_id: "1",
    section_id: "1",
    created_at: "2020-01-01T00:00:00.000Z",
    updated_at: "2020-01-01T00:00:00.000Z",
    version: 1,
  },
];

export function TaskList() {
  return (
    <div>
      <Listbox
        variant="bordered"
        color="default"
        selectionMode="none"
        isVirtualized
        // className="mx-6 w-full"
        // label={"Select from 1000 items"}
        virtualization={{
          maxListboxHeight: 400,
          itemHeight: 40,
        }}
      >
        {tasks.map((item) => (
          <ListboxItem
            startContent={<SmallControlButton />}
            key={item.id}
            textValue={item.name}
          >
            <TaskItem task={item} />
          </ListboxItem>
        ))}
      </Listbox>
    </div>
  );
}
