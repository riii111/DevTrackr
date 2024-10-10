import dynamic from 'next/dynamic';

const DynamicAccountMenu = dynamic(() => import('@/components/molecules/core/AccountMenu'), {
    ssr: false,
    loading: () => <div className="w-8 h-8 bg-main-bg rounded-full animate-pulse"></div>
});

const DynamicPageTitle = dynamic(() => import('@/components/molecules/PageTitle'), {
    ssr: false,
    loading: () => <div className="h-6 w-32 bg-main-bg animate-pulse rounded"></div>
});

const LayoutDashboardHeader = () => {
    return (
        <header className="flex items-center justify-between px-4 py-2 border-b border-gray-400">
            <DynamicPageTitle />
            <DynamicAccountMenu />
        </header>
    );
};

export default LayoutDashboardHeader;