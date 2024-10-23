import dynamic from 'next/dynamic';
import UserMenu from '@/components/features/users/menu/UserMenu';
import { getMeDetails } from '@/lib/api/user';

const DynamicPageTitle = dynamic(() => import('@/components/core/PageTitle'), {
    ssr: false,
    loading: () => <div className="h-6 w-32 bg-gray-200 animate-pulse rounded"></div>
});

const LayoutDashboardHeader = async () => {
    const userResponse = await getMeDetails();

    return (
        <header className="flex items-center justify-between px-4 py-2 border-b border-gray-400">
            <DynamicPageTitle />
            <UserMenu initialUserData={userResponse} />
        </header>
    );
};

export default LayoutDashboardHeader;
