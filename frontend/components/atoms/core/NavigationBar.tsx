import Link from "next/link";
import { IoIosSettings } from "react-icons/io";
import { usePathname } from 'next/navigation';

interface NavigationBarProps {
    menus: {
        [key: string]: Array<{
            name: string;
            path: string;
            icon: React.ReactNode;
        }>;
    }
};

const NavigationBar = ({ menus }: NavigationBarProps) => {
    const pathname = usePathname();

    return (
        <nav className="h-full w-1/4 bg-[#060606] shadow flex flex-col border-gray-50">
            <h2 className="text-gray-500 mb-4 gap-2 text-lg flex items-center py-6">
                <IoIosSettings className="text-2xl text-[#E65F2B]" />
                <span className="text-[#FFFFFF]">設定</span>
            </h2>
            {
                Object.entries(menus).map(([category, items], index) => (
                    <div key={category} className={`p-2 rounded-[17px] shadow`}>
                        <h3 className="text-gray-500 mb-4 gap-4">{category}</h3>
                        <ul className="flex flex-col gap-4 ">
                            {items.map((menu) => {
                                const isActive = pathname === menu.path;
                                return (
                                    <li key={menu.path} className="gap-4">
                                        <Link
                                            href={menu.path}
                                            className={`flex items-center gap-4 rounded-md group ${isActive ? 'bg-[#E65F2B] text-white' : 'hover:bg-gray-300'
                                                }`}
                                        >
                                            <span className={`${isActive ? 'text-white' : 'group-hover:text-[#E65F2B]'} transition-colors`}>{menu.icon}</span>
                                            <span className={`${isActive ? 'text-white' : 'group-hover:text-[#E65F2B]'} transition-colors`}>{menu.name}</span>
                                        </Link>
                                    </li>
                                );
                            })}
                        </ul>
                    </div>
                ))
            }
        </nav>
    );
};

export default NavigationBar;