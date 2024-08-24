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
            path: "/time-tracking",
            icon: <FaTasks />,
        },
        {
            name: "カレンダー",
            path: "/calendar",
            icon: <FaRegCalendarAlt />,
        },
        {
            name: "スキル分析",
            path: "/skill-analysis",
            icon: <BiAnalyse />,
        },
        {
            name: "収益・時給管理",
            path: "/revenue",
            icon: <FaMoneyBillTrendUp />,
        },
        {
            name: "振り返り",
            path: "/retrospective",
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
