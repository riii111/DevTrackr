import { getServerSideProjects } from "@/lib/hooks/useProjectsApi";
import SWRConfigWrapper from "@/app/dashboard/time-tracking/SWRConfigWrapper";
import TimeTrackingClientComponents from "@/app/dashboard/time-tracking/TimeTrackingClientComponents";
import TimeTrackingContent from "@/app/dashboard/time-tracking/TimeTrackingContent";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function TimeTrackingPage() {
    const projectsData = await getServerSideProjects();

    return (
        <SWRConfigWrapper fallback={{ '/projects/': projectsData }}>
            <TimeTrackingClientComponents />
            <TimeTrackingContent bgColor={bgColor} />
        </SWRConfigWrapper>
    );
}