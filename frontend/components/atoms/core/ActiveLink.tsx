"use client";

import Link from "next/link";
import { usePathname } from 'next/navigation';

interface ActiveLinkProps {
    href: string;
    name: string;
    icon: React.ReactNode;
}

const ActiveLink = ({ href, name, icon }: ActiveLinkProps) => {
    const pathname = usePathname();
    const isActive = pathname === href;

    return (
        <Link
            href={href}
            className={`flex items-center gap-4 rounded-md group ${isActive ? 'bg-white text-[#E65F2B]' : 'hover:bg-gray-300'
                }`}
        >
            <span className={`${isActive ? 'text-accent' : 'text-white group-hover:text-accent'
                } transition-colors`}>{icon}</span>
            <span className={`${isActive ? 'text-accent' : 'text-white group-hover:text-accent'
                } transition-colors`}>{name}</span>
        </Link>
    );
};

export default ActiveLink;