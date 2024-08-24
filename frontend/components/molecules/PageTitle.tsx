"use client";
import { usePathname } from "next/navigation";

const getPageTitle = (pathname: string) => {
  const pathSegments = pathname.split("/").filter(Boolean);
  const lastSegment = pathSegments[pathSegments.length - 1];

  const titleMap: { [key: string]: string } = {
    "profile": "プロフィール",
    "password": "パスワード",
    "notifications": "通知",
    "project-list": "案件プロジェクト一覧",
    "user": "ユーザー管理",
    "project-category": "プロジェクトカテゴリー",
  };

  return titleMap[lastSegment] || '設定';
};

const PageTitle = () => {
  const pathname = usePathname();
  const pageTitle = getPageTitle(pathname);

  return <h1 className="text-xl font-semibold">{pageTitle}</h1>;
};

export default PageTitle;