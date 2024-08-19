import Link from "next/link";

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
    return (
        <nav className="h-full w-1/4 bg-[#060606] shadow flex flex-col border-gray-50">
            {
                Object.entries(menus).map(([category, items], index) => (
                    <div key={category} className={`p-2 rounded-[17px] shadow`}>
                        <h3 className="text-gray-500 mb-4 gap-4">{category}</h3>
                        <ul className="flex flex-col gap-4">
                            {items.map((menu) => (
                                <li key={menu.path} className="gap-4">
                                    <Link href={menu.path} className="flex items-center gap-4 hover:bg-gray-300 rounded-md group">
                                        <span className="group-hover:text-[#E65F2B] transition-colors">{menu.icon}</span>
                                        <span className="group-hover:text-[#E65F2B] transition-colors">{menu.name}</span>
                                    </Link>
                                </li>
                            ))}
                        </ul>
                    </div>
                ))
            }
        </nav>
    );
};

export default NavigationBar;