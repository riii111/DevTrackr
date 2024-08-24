import { LuLayoutDashboard } from "react-icons/lu";
import { FaTasks } from "react-icons/fa";
import { FaRegCalendarAlt } from "react-icons/fa";
import { BiAnalyse } from "react-icons/bi";
import { FaMoneyBillTrendUp } from "react-icons/fa6";
import { MdRateReview } from "react-icons/md";
import { MdDashboardCustomize } from "react-icons/md";

import NavigationBar from "@/components/molecules/NavigationBar";


const LayoutDashboardNavigation = () => {

    const dashboard = [
        {
            name: "ダッシュボード",
            path: "/dashboard",
            icon: <LuLayoutDashboard />,
        },
        {
            name: "勤怠",
            path: "/dashboard/time-tracking",
            icon: <FaTasks />,
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
        {
            name: "収益・時給管理",
            path: "/dashboard/revenue",
            icon: <FaMoneyBillTrendUp />,
        },
        {
            name: "振り返り",
            path: "/dashboard/retrospective",
            icon: <MdRateReview />,
        },
    ];

    const menus = {
        "ダッシュボード": dashboard,
    }

    return (
        <NavigationBar menus={menus} title={{ name: "DevTrackr", icon: <MdDashboardCustomize /> }} />
    );
};

export default LayoutDashboardNavigation;
