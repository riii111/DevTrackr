import dynamic from 'next/dynamic';

const DynamicAccountMenu = dynamic(() => import('@/components/molecules/core/AccountMenu'), {
    ssr: false,
    loading: () => <div className="w-8 h-8 bg-gray-200 rounded-full animate-pulse"></div>
});

const DynamicPageTitle = dynamic(() => import('@/components/molecules/PageTitle'), {
    ssr: false,
    loading: () => <div className="h-6 w-32 bg-gray-200 animate-pulse rounded"></div>
});

const LayoutDashboardHeader = () => {
    return (
        <header className="w-full h-16 border-b border-gray-400">
            <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center h-full">
                <DynamicPageTitle />
                <div className="ml-auto">
                    <DynamicAccountMenu />
                </div>
            </nav>
        </header>
    );
};

export default LayoutDashboardHeader;