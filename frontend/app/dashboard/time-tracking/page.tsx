import TimeTrackingContent from "@/app/dashboard/time-tracking/TimeTrackingContent";
import TimeTrackingClientComponents from "./TimeTrackingClientComponents";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function TimeTrackingPage() {
    return (
        <>
            <TimeTrackingClientComponents />
            <TimeTrackingContent bgColor={bgColor} />
        </>
    );
}