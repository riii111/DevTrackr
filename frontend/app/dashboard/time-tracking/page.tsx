import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import TimeTrackingContent from "@/app/dashboard/time-tracking/TimeTrackingContent";
import TimeTrackingClientComponents from "./TimeTrackingClientComponents";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function TimeTrackingPage() {
    const { getProjects } = useProjectsApi();
    const projectsData = await getProjects();

    return (
        <>
            <TimeTrackingClientComponents />
            <TimeTrackingContent bgColor={bgColor} projects={projectsData.data} />
        </>
    );
}