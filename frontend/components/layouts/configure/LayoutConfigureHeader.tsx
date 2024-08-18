// import React from "react";
// import { MdAccountCircle } from "react-icons/md";
// import { RiLockPasswordFill } from "react-icons/ri";
// import { VscBellDot } from "react-icons/vsc";

// interface LayoutConfigureHeaderProps {
//   title?: string;
// }

// const individual = [
//   {
//     name: "プロフィール",
//     path: "/setting/individual/profile",
//     icon: <MdAccountCircle />,
//   },
//   {
//     name: "パスワード",
//     path: "/setting/individual/password",
//     icon: <RiLockPasswordFill />,
//   },
//   {
//     name: "通知",
//     path: "/setting/individual/notification",
//     icon: <VscBellDot />,
//   },
// ];

const LayoutConfigureHeader: React.FC<LayoutConfigureHeaderProps> = ({
  title,
}) => {
  // 仮の実装
  return (
    <header className="bg-gray-100 p-4">
      <h1 className="text-2xl font-bold">{title || 'デフォルトタイトル'}</h1>
      <nav className="mt-2">
        <ul className="flex space-x-4">
          <li><a href="#" className="text-blue-500 hover:underline">メニュー1</a></li>
          <li><a href="#" className="text-blue-500 hover:underline">メニュー2</a></li>
          <li><a href="#" className="text-blue-500 hover:underline">メニュー3</a></li>
        </ul>
      </nav>
    </header>
  );
};

export default LayoutConfigureHeader;
