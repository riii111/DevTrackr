import ActiveLink from '@/components/atoms/core/ActiveLink';

interface NavigationBarProps {
    menus: {
        [key: string]: Array<{
            name: string;
            path: string;
            icon: React.ReactNode;
        }>;
    },
    title: {
        name: string;
        icon: React.ReactNode;
    }
};

const NavigationBar = ({ menus, title }: NavigationBarProps) => {
    return (
        <nav className="h-full w-1/5 bg-[#060606] shadow flex flex-col border-gray-50">
            <h2 className="text-gray-500 mb-4 gap-4 text-2xl flex items-center py-6">
                <span className="text-[#E65F2B]">{title.icon}</span>
                <span className="text-[#FFFFFF]">{title.name}</span>
            </h2>
            {Object.entries(menus).map(([category, items], index) => (
                <div key={category} className="p-2 text-lg rounded-[26px] shadow">
                    <h3 className="text-gray-500 mb-4 gap-4">{category}</h3>
                    <ul className="flex flex-col gap-6">
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