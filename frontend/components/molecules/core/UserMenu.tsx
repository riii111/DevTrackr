"use client";
import { MdAccountCircle } from 'react-icons/md';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { useAuthApi } from '@/lib/hooks/useAuthApi';
import { useRouter } from 'next/navigation';

const UserMenu: React.FC = () => {
    const { logout } = useAuthApi();
    const router = useRouter();

    const handleLogout = async () => {
        try {
            await logout();
            // ログアウト後、ログインページなどにリダイレクト
            router.push('/auth');
        } catch (error) {
            console.error('ログアウトに失敗しました', error);
        }
    };

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <button className="text-gray-500 hover:text-gray-700 focus:outline-none">
                    <span className="sr-only">アカウントメニューを開く</span>
                    <MdAccountCircle className="h-8 w-8" />
                </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56">
                <div className="px-4 py-2 text-sm text-gray-700">
                    <div className="flex items-center space-x-2">
                        <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
                            アド
                        </div>
                        <span>アドミン1 ユーザー</span>
                    </div>
                </div>
                <DropdownMenuItem onSelect={handleLogout}>
                    ログアウト
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    );
};

export default UserMenu;