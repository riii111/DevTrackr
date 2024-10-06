"use client";

import { usePathname } from 'next/navigation';
import { Button } from "@/components/ui/button";
import Link from "next/link";
import { cn } from "@/lib/utils";

interface ActiveLinkProps {
    href: string;
    name: string;
    icon: React.ReactNode;
}

const ActiveLink = ({ href, name, icon }: ActiveLinkProps) => {
    const pathname = usePathname();
    const isActive = pathname === href;

    return (
        <Button
            variant="ghost"
            className={cn(
                "w-full justify-start gap-2 text-lg",
                isActive
                    ? "bg-white text-accent"
                    : "text-white hover:bg-secondary/10 hover:text-accent"
            )}
            asChild
        >
            <Link href={href}>
                <span className="transition-colors">{icon}</span>
                <span className="transition-colors">{name}</span>
            </Link>
        </Button>
    );
};

export default ActiveLink;