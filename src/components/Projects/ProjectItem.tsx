import { Project, Section } from "../../types";
import { formatDate } from "../../utils/formatDate";

export function ProjectItem({ project }: { project: any }) {
  return <div>{project.name}</div>;
}
