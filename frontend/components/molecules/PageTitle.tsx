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
    // "revenue": "収益・時給管理",
    "retrospective": "振り返り",
  };

  const configureTitleMap: { [key: string]: string } = {
    "profile": "プロフィール",
    "password": "パスワード",
    "notifications": "通知",
    "project-list": "案件プロジェクト一覧",
    "user": "ユーザー管理",
    "project-category": "プロジェクトカテゴリー",
  };

  if (firstSegment === "dashboard") {
    return dashboardTitleMap[lastSegment] || "にゃほにゃほ";
  } else if (firstSegment === "configure") {
    return configureTitleMap[lastSegment] || "設定";
  }

  return "ダッシュボード"; // デフォルトタイトル
};

const PageTitle = () => {
  const pathname = usePathname();
  const pageTitle = getPageTitle(pathname);

  return <h1 className="flex justify-start text-xl font-semibold text-text-primary">{pageTitle}</h1>;
};

export default PageTitle;