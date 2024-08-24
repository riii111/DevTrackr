import AtomsButtonWithIcon from "@/components/atoms/button/AtomsButtonWithIcon";
import { GoPlus } from "react-icons/go";

const bgColor = "bg-white bg-opacity-30 backdrop-filter backdrop-blur-sm";

export default function TimeTrackingPage() {
    return (
        <>
            <div className="flex justify-start">
                <AtomsButtonWithIcon
                    icon={GoPlus}
                    text="勤怠を追加"
                    // btnColor="bg-white"
                    iconColor="text-black group-hover:text-[#E65F2B]"
                    textColor="text-black group-hover:text-[#E65F2B]"
                    rounded={6}
                    loading={false}
                    disabled={false}
                // onClick={() => { }}
                />
            </div>
            <br />
            <div className={`p-6 rounded-lg ${bgColor}`}>
                <h1 className="text-2xl font-bold mb-4">勤怠</h1>
                <p>ここに勤怠の情報が表示されます。</p>
            </div>
        </>
    );
}