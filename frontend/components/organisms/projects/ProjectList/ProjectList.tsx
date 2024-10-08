import { Project } from "@/types/project";

interface ProjectListProps {
    projects: Project[] | undefined;
}

export const ProjectList: React.FC<ProjectListProps> = ({ projects }) => {
    if (!projects || projects.length === 0) {
        return <p>プロジェクトがありません。</p>;
    }

    return (
        <ul>
            {projects.map((project) => (
                <li key={project.id}>{project.title}</li>
            ))}
        </ul>
    );
};