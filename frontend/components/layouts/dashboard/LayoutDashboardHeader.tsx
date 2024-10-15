import dynamic from 'next/dynamic';
import UserMenu from '@/components/organisms/users/UserMenu';
import { useUserApi } from '@/lib/hooks/useUserApi';
const DynamicPageTitle = dynamic(() => import('@/components/molecules/PageTitle'), {
    ssr: false,
    loading: () => <div className="h-6 w-32 bg-gray-200 animate-pulse rounded"></div>
});

const LayoutDashboardHeader = async () => {
    const { getMeDetails } = useUserApi();
    const user = await getMeDetails();

    return (
        <header className="flex items-center justify-between px-4 py-2 border-b border-gray-400">
            <DynamicPageTitle />
            <UserMenu initialUserData={user} />
        </header>
    );
};

export default LayoutDashboardHeader;