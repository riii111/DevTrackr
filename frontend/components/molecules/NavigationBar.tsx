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
        <nav className="h-full w-1/5 bg-navigation-bg shadow flex flex-col border-secondary">
            <h2 className="text-secondary mb-4 gap-4 text-2xl flex items-center py-6">
                <span className="text-accent">{title.icon}</span>
                <span className="text-white">{title.name}</span>
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