import { ProjectTable } from "@/components/organisms/projects/ProjectTable/ProjectTable";
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import { Project } from "@/types/project";

interface TimeTrackingContentProps {
    bgColor: string;
    projects: Project[];
}

export default async function TimeTrackingContent({ bgColor }: { bgColor: string }) {
    const { getProjects } = useProjectsApi();
    const projects = await getProjects();
    return (
        <>
            {projects && (
                <div className={`p-6 rounded-lg ${bgColor} text-text-primary`}>
                    <h1 className="text-2xl font-bold mb-4">勤怠</h1>
                    <ProjectTable projects={projects} />
                </div>
            )}
        </>
    );
}
