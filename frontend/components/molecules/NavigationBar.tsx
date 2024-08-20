import { IoIosSettings } from "react-icons/io";
import ActiveLink from '@/components/atoms/core/ActiveLink';

interface NavigationBarProps {
    menus: {
        [key: string]: Array<{
            name: string;
            path: string;
            icon: React.ReactNode;
        }>;
    },
    title: string;
};

const NavigationBar = ({ menus, title }: NavigationBarProps) => {
    return (
        <nav className="h-full w-1/5 bg-[#060606] shadow flex flex-col border-gray-50">
            <h2 className="text-gray-500 mb-4 gap-2 text-lg flex items-center py-6">
                <IoIosSettings className="text-2xl text-[#E65F2B]" />
                <span className="text-[#FFFFFF]">{title}</span>
            </h2>
            {Object.entries(menus).map(([category, items], index) => (
                <div key={category} className={`p-2 rounded-[26px] shadow`}>
                    <h3 className="text-gray-500 mb-4 gap-4">{category}</h3>
                    <ul className="flex flex-col gap-4">
                        {items.map((menu) => (
                            <li key={menu.path} className="gap-4">
                                <ActiveLink href={menu.path} name={menu.name} icon={menu.icon} />
                            </li>
                        ))}
                    </ul>
                </div>
            ))}
        </nav>
    );
};

export default NavigationBar;