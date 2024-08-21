"use client";
import React, { Fragment } from 'react';
import { Menu, Transition } from '@headlessui/react';
import { MdAccountCircle } from 'react-icons/md';

const AccountMenu: React.FC = () => {
    return (
        <Menu as="div" className="relative">
            <Menu.Button className="text-gray-500 hover:text-gray-700 focus:outline-none">
                <span className="sr-only">アカウントメニューを開く</span>
                <MdAccountCircle className="h-8 w-8" />
            </Menu.Button>
            <Transition
                as={Fragment}
                enter="transition ease-out duration-100"
                enterFrom="transform opacity-0 scale-95"
                enterTo="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leaveFrom="transform opacity-100 scale-100"
                leaveTo="transform opacity-0 scale-95"
            >
                <Menu.Items className="absolute right-0 mt-2 w-56 origin-top-right bg-white rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                    <div className="py-1">
                        <div className="px-4 py-2 text-sm text-gray-700">
                            <div className="flex items-center space-x-2">
                                <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
                                    アド
                                </div>
                                <span>アドミン1 ユーザー</span>
                            </div>
                        </div>
                        <Menu.Item>
                            {({ active }) => (
                                <a
                                    href="#"
                                    className={`block px-4 py-2 text-sm ${active ? 'bg-gray-100 text-gray-900' : 'text-gray-700'}`}
                                >
                                    ログアウト
                                </a>
                            )}
                        </Menu.Item>
                    </div>
                </Menu.Items>
            </Transition>
        </Menu >
    );
};

export default AccountMenu;