import { MdAccountCircle } from "react-icons/md";
import { RiLockPasswordFill } from "react-icons/ri";
import { VscBellDot } from "react-icons/vsc";

import { FaTasks } from "react-icons/fa";
import { CiUser } from "react-icons/ci";
import { MdLabelImportant } from "react-icons/md";

import NavigationBar from "@/components/atoms/core/NavigationBar";


// const LayoutConfigureNavigation = ({organization}) => {
const LayoutConfigureNavigation = () => {

  const individual = [
    {
      name: "プロフィール",
      path: "/configure/individual/profile",
      icon: <MdAccountCircle />,
    },
    {
      name: "パスワード",
      path: "/configure/individual/password",
      icon: <RiLockPasswordFill />,
    },
    {
      name: "通知",
      path: "/configure/individual/notification",
      icon: <VscBellDot />,
    },
  ];

  const organization = [
    {
      name: "案件プロジェクト一覧",
      path: "/configure/organization/project-list",
      icon: <FaTasks />,
    },
    {
      name: "ユーザー管理",
      path: "/configure/organization/user",
      icon: <CiUser />,
    },
    {
      name: "プロジェクトカテゴリー",
      path: "/configure/organization/project-category",
      icon: <MdLabelImportant />,
    },
  ];

  const menus = {
    "個人設定": individual,
    "組織設定": organization,
  }

  return (
    <NavigationBar menus={menus} title="設定" />
  );
};

export default LayoutConfigureNavigation;
