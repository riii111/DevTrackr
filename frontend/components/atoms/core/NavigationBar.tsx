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
        <nav className="h-full w-64 bg-gray-100 flex flex-col">
            {
                Object.entries(menus).map(([category, items], index) => (
                    <div key={category} className={`${index !== 0 ? 'border-t border-gray-300 pt-4' : ''} my-4`}>
                        <h3 className="text-sm text-gray-500 mb-2">{category}</h3>
                        <ul>
                            {items.map((menu) => (
                                <li key={menu.path}>
                                    <Link href={menu.path} className="flex items-center gap-2 hover:bg-gray-100 my-2 rounded-md">
                                        {menu.icon}
                                        {menu.name}
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