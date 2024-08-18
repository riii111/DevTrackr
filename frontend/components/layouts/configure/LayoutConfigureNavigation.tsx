import { MdAccountCircle } from "react-icons/md";
import { RiLockPasswordFill } from "react-icons/ri";
import { VscBellDot } from "react-icons/vsc";

import { FaTasks } from "react-icons/fa";
import { CiUser } from "react-icons/ci";
import { MdLabelImportant } from "react-icons/md";

import NavigationBar from "@/components/atoms/core/NavigationBar";


const LayoutConfigureNavigation = () => {

  const individual = [
    {
      name: "プロフィール",
      path: "/setting/individual/profile",
      icon: <MdAccountCircle />,
    },
    {
      name: "パスワード",
      path: "/setting/individual/password",
      icon: <RiLockPasswordFill />,
    },
    {
      name: "通知",
      path: "/setting/individual/notification",
      icon: <VscBellDot />,
    },
  ];

  const organization = [
    {
      name: "案件プロジェクト一覧",
      path: "/settings/organization/project",
      icon: <FaTasks />,
    },
    {
      name: "ユーザー管理",
      path: "/settings/organization/user",
      icon: <CiUser />,
    },
    {
      name: "プロジェクトカテゴリー",
      path: "/settings/organization/project-category",
      icon: <MdLabelImportant />,
    },
  ];

  const menus = {
    "個人設定": individual,
    "組織設定": organization,
  }

  return (
    <NavigationBar menus={menus} />
  );
};

export default LayoutConfigureNavigation;
