import { ProjectList } from "@/components/organisms/projects/ProjectList/ProjectList";
import { Project } from "@/types/project";

interface TimeTrackingContentProps {
    bgColor: string;
    projects: Project[];
}

export default function TimeTrackingContent({ bgColor, projects }: TimeTrackingContentProps) {
    return (
        <div className={`p-6 rounded-lg ${bgColor} text-text-primary`}>
            <h1 className="text-2xl font-bold mb-4">勤怠</h1>
            <ProjectList projects={projects} />
        </div>
    );
}
