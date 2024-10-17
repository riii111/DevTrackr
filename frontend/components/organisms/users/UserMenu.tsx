'use client';

import { MdAccountCircle } from 'react-icons/md';
import { IoIosArrowDown } from "react-icons/io";
import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { useAuthApi } from '@/lib/hooks/useAuthApi';
import { User } from '@/types/user';
import { toast } from '@/lib/hooks/use-toast';

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
            toast({
                title: 'ログアウトしました',
                variant: 'default',
            });
        } catch (error) {
            console.error('ログアウトに失敗しました', error);
        }
    };

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <button className="flex items-center space-x-2 focus:outline-none bg-white rounded-full px-2 shadow-sm">
                    {initialUserData.avatar ? (
                        <Image
                            src={initialUserData.avatar}
                            alt={initialUserData.username}
                            width={32}
                            height={32}
                            className="rounded-full"
                        />
                    ) : (
                        <MdAccountCircle size={36} className="text-gray-500" />
                    )}
                    <div className="text-left">
                        <p className="text-primary text-sm">{initialUserData.username}</p>
                        <p className="text-sm text-gray-400">{initialUserData.role}</p>
                    </div>
                    <IoIosArrowDown size={16} style={{ color: "black" }} />
                </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56">
                <DropdownMenuItem onSelect={() => router.push('/dashboard/profile')}>
                    プロフィール設定
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={handleLogout}>
                    ログアウト
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    );
}