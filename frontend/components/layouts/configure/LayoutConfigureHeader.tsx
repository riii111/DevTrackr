"use client";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";

import AccountMenu from '@/components/molecules/core/AccountMenu';


const getPageTitle = (pathname: string) => {
  console.log("pathname", pathname);
  const pathSegments = pathname.split("/").filter(Boolean);
  console.log("pathSegments", pathSegments);
  const lastSegment = pathSegments[pathSegments.length - 1];
  console.log("lastSegment", lastSegment);

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

const LayoutConfigureHeader = () => {


  const pathname = usePathname();
  const [pageTitle, setPageTitle] = useState<string | undefined>();

  useEffect(() => {
    setPageTitle(getPageTitle(pathname));
  }, [pathname]);

  return (
    <header className="bg-[#EBDFD7] w-full h-16 border-b border-gray-400">
      <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center h-full">
        <h1 className="text-xl font-semibold">{pageTitle}</h1>
        <div className="ml-auto">
          <AccountMenu />
        </div>
      </nav>
    </header>
  );
};

export default LayoutConfigureHeader;
