import { useState } from "react";
import { Input, Listbox, ListboxItem } from "@heroui/react";
import { ProjectItem } from "./ProjectItem";

interface Project {
  id: string;
  name: string;
  duration?: string;
  isActive?: boolean;
}

const ListboxWrapper = ({ children }: { children: React.ReactNode }) => (
  <div className="w-full max-w-[260px] border-small px-1 py-2 rounded-small border-default-200 dark:border-default-100">
    {children}
  </div>
);

export function ProjectList() {
  const [searchQuery, setSearchQuery] = useState("");

  // Mock data - replace with real data later
  const projects: Project[] = [
    { id: "1", name: "My Task", duration: "2:30" },
    { id: "2", name: "Business Development", duration: "2:30", isActive: true },
    { id: "3", name: "Greysheet", duration: "2:30" },
    { id: "4", name: "Onesuite Design", duration: "2:30" },
    { id: "5", name: "OneDesk", duration: "2:30" },
    { id: "6", name: "Makkan Builders", duration: "2:30" },
    { id: "7", name: "Home Vision CRM", duration: "2:30" },
    { id: "8", name: "Fuel Cells Works", duration: "2:30" },
    { id: "9", name: "Holiday Trip Vista", duration: "2:30" },
    {
      id: "10",
      name: "Business Development",
      duration: "2:30",
      isActive: true,
    },
    { id: "11", name: "Greysheet", duration: "2:30" },
    { id: "12", name: "Onesuite Design", duration: "2:30" },
    { id: "13", name: "OneDesk", duration: "2:30" },
    { id: "14", name: "Makkan Builders", duration: "2:30" },
    { id: "15", name: "Home Vision CRM", duration: "2:30" },
    { id: "16", name: "Fuel Cells Works", duration: "2:30" },
    { id: "17", name: "Holiday Trip Vista", duration: "2:30" },
  ];

  const filteredProjects = projects.filter((project) =>
    project.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-gray-200">
        <Input
          type="text"
          placeholder="Search project"
          variant="bordered"
          color="primary"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      <div className="flex-1 overflow-auto py-2">
        <div className="px-4 py-2">
          <h3 className="text-sm font-medium text-gray-500">Projects</h3>
        </div>
        <Listbox
          variant="faded"
          color="primary"
          isVirtualized
          virtualization={{
            maxListboxHeight: 400,
            itemHeight: 40,
          }}
        >
          {filteredProjects.map((item) => (
            <ListboxItem key={item.id} textValue={item.name}>
              <ProjectItem project={item} />
            </ListboxItem>
          ))}
        </Listbox>
      </div>
    </div>
  );
}
