"use client";
import { usePathname } from "next/navigation";

const getPageTitle = (pathname: string) => {
  const pathSegments = pathname.split("/").filter(Boolean);
  const firstSegment = pathSegments[0];
  const lastSegment = pathSegments[pathSegments.length - 1];

  const dashboardTitleMap: { [key: string]: string } = {
    "dashboard": "ダッシュボード",
    "projects": "開発プロジェクト一覧",
    "calendar": "カレンダー",
    "skill-analysis": "スキル分析",
  };

  if (firstSegment === "dashboard") {
    return dashboardTitleMap[lastSegment] || "にゃほにゃほ";
  }

  return "ダッシュボード"; // デフォルトタイトル
};

const PageTitle = () => {
  const pathname = usePathname();
  const pageTitle = getPageTitle(pathname);

  return <h1 className="flex justify-start text-xl font-semibold text-text-primary">{pageTitle}</h1>;
};

export default PageTitle;