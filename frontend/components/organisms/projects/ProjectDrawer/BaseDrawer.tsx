"use client";

import {
    Drawer,
    DrawerContent,
    DrawerHeader,
    DrawerTitle,
    DrawerDescription,
} from "@/components/ui/drawer";

interface BaseDrawerProps {
    isOpen: boolean;
    onOpenChange: (open: boolean) => void;
    title: string;
    children: React.ReactNode;
    onClose: () => void;
}

export function BaseDrawer({ isOpen, onOpenChange, title, children, onClose }: BaseDrawerProps) {
    return (
        <Drawer open={isOpen} onOpenChange={onOpenChange} onClose={onClose}>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle>{title}</DrawerTitle>
                    <DrawerDescription>{children}</DrawerDescription>
                </DrawerHeader>
            </DrawerContent>
        </Drawer>
    );
}