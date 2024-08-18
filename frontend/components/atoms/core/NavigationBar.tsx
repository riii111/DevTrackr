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
                Object.entries(menus).map(([category, items]) => (
                    <div key={category} className="mb-4">
                        <h3 className="text-sm text-gray-500 mb-2">{category}</h3>
                        <ul>
                            {items.map((menu) => (
                                <li key={menu.path}>
                                    <Link href={menu.path} className="flex items-center gap-2 hover:bg-gray-100 rounded-md">
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