import TimeTrackingContent from "./TimeTrackingContent";
import TimeTrackingClientComponents from "./TimeTrackingClientComponents";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default function TimeTrackingPage() {
    return (
        <>
            <TimeTrackingClientComponents />
            <TimeTrackingContent bgColor={bgColor} />
        </>
    );
}