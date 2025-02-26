import { LuLayoutDashboard } from "react-icons/lu";
import { FaTasks } from "react-icons/fa";
import { FaRegCalendarAlt } from "react-icons/fa";
import { BiAnalyse } from "react-icons/bi";
import { BsBuildings } from "react-icons/bs";
// import { FaMoneyBillTrendUp } from "react-icons/fa6";
// import { MdRateReview } from "react-icons/md";
import { MdDashboardCustomize } from "react-icons/md";

import { ScrollArea } from "@/components/ui/scroll-area";
import ActiveLink from "@/components/core/ActiveLink";

const LayoutDashboardNavigation = () => {
    const dashboard = [
        {
            name: "ダッシュボード",
            path: "/dashboard",
            icon: <LuLayoutDashboard />,
        },
        {
            name: "プロジェクト一覧",
            path: "/dashboard/projects",
            icon: <FaTasks />,
        },
        {
            name: "企業一覧",
            path: "/dashboard/companies",
            icon: <BsBuildings />,
        },
        {
            name: "カレンダー",
            path: "/dashboard/calendar",
            icon: <FaRegCalendarAlt />,
        },
        {
            name: "スキル分析",
            path: "/dashboard/skill-analysis",
            icon: <BiAnalyse />,
        },
    ];

    return (
        <nav className="h-full w-56 flex-shrink-0 bg-navigation-bg shadow flex flex-col border-secondary">
            <h2 className="text-secondary mb-4 gap-4 text-2xl flex items-center py-6 px-4">
                <MdDashboardCustomize className="text-accent" />
                <span className="text-white">DevTrackr</span>
            </h2>
            <ScrollArea className="flex-1 px-2">
                <div className="space-y-2">
                    {dashboard.map((item) => (
                        <ActiveLink
                            key={item.path}
                            href={item.path}
                            name={item.name}
                            icon={item.icon}
                        />
                    ))}
                </div>
            </ScrollArea>
        </nav>
    );
};

export default LayoutDashboardNavigation;
