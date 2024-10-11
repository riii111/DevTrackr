'use client';

import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { useAuthApi } from '@/lib/hooks/useAuthApi';
import { User } from '@/types/user';

interface UserMenuProps {
    initialUserData: User;
}

export default function UserMenu({ initialUserData }: UserMenuProps) {
    const { logout } = useAuthApi();
    const router = useRouter();

    const handleLogout = async () => {
        try {
            await logout();
            router.push('/auth');
        } catch (error) {
            console.error('ログアウトに失敗しました', error);
        }
    };

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <button className="flex items-center space-x-2 focus:outline-none">
                    <Image
                        src={initialUserData.icon}
                        alt={initialUserData.username}
                        width={40}
                        height={40}
                        className="rounded-full"
                    />
                    <div className="text-left">
                        <p className="font-semibold">{initialUserData.username}</p>
                        <p className="text-sm text-gray-500">{initialUserData.role}</p>
                    </div>
                </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56">
                <DropdownMenuItem onSelect={() => router.push('/profile')}>
                    プロフィール
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => router.push('/settings')}>
                    設定
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={handleLogout}>
                    ログアウト
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    );
}