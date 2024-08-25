import TimeTrackingContent from "./TimeTrackingContent";
import TimeTrackingClientComponents from "./TimeTrackingClientComponents";

const bgColor = "bg-white bg-opacity-30 backdrop-filter backdrop-blur-sm";

export default function TimeTrackingPage() {
    return (
        <>
            <TimeTrackingClientComponents />
            <TimeTrackingContent bgColor={bgColor} />
        </>
    );
}