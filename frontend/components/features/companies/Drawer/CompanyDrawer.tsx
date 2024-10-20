"use client";

import { Suspense } from 'react';
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription } from "@/components/ui/sheet"
import { useSearchParams, useRouter } from "next/navigation";
import React, { useEffect, useState, useMemo } from "react";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { CompanyDrawerBody } from "@/components/features/companies/Drawer/content/CompanyDrawerBody"

const DRAWER_WIDTH = 640
const MAIN_DRAWER_FULL_MIN_WIDTH = 1000
const SUB_DRAWER_WIDTH = 520

const windowResizeObserver = (setWindowWidth: React.Dispatch<React.SetStateAction<number>>) => {
    setWindowWidth(window.innerWidth);
};

export const CompanyDrawer = React.memo(() => {
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <CompanyDrawerContent />
        </Suspense>
    );
});

const CompanyDrawerContent = () => {
    const router = useRouter();
    const searchParams = useSearchParams();
    const drawerStore = useDrawerStore();
    const mainState = drawerStore.drawerState.main
    const subState = drawerStore.drawerState.sub
    const [windowWidth, setWindowWidth] = useState(0)

    const selectedCompanyId = searchParams.get("companyId");

    const mainDrawerWidth = useMemo(() => {
        if (!drawerStore.isFullScreen) {
            return DRAWER_WIDTH;
        }
        return windowWidth - SUB_DRAWER_WIDTH > MAIN_DRAWER_FULL_MIN_WIDTH
            ? windowWidth - SUB_DRAWER_WIDTH
            : MAIN_DRAWER_FULL_MIN_WIDTH;
    }, [drawerStore.isFullScreen, windowWidth]);

    const containerWidth = useMemo(() => {
        return subState.isOpen ? windowWidth : mainDrawerWidth;
    }, [subState.isOpen, windowWidth, mainDrawerWidth]);

    useEffect(() => {
        windowResizeObserver(setWindowWidth);
        window.addEventListener('resize', () => windowResizeObserver(setWindowWidth));

        return () => {
            window.removeEventListener('resize', () => windowResizeObserver(setWindowWidth));
        };
    }, []);

    const onUpdateModelValue = (isOpen: boolean) => {
        if (!isOpen) {
            drawerStore.handleClose("main");
            router.push("/dashboard/companies");
        }
    };

    return (
        <div className="absolute inset-0 overflow-hidden z-50">
            <div className="absolute inset-0 overflow-hidden">
                <div className="pointer-events-none absolute inset-y-0 right-0 flex max-w-full pl-10">
                    <Sheet open={mainState.isOpen} onOpenChange={onUpdateModelValue}>
                        <SheetContent
                            side="right"
                            className="p-0 w-full sm:max-w-full bg-dialog-bg text-primary"
                            style={{ width: containerWidth }}
                        >
                            <SheetHeader>
                                <SheetTitle className="text-primary font-bold">企業詳細</SheetTitle>
                                <SheetDescription>企業の詳細情報を表示する</SheetDescription>
                            </SheetHeader>
                            <div className="flex h-screen">
                                <div
                                    className="h-full overflow-y-auto"
                                    style={{ width: `${mainDrawerWidth}px` }}
                                >
                                    <CompanyDrawerBody drawerType="main" selectedCompanyId={selectedCompanyId} />
                                </div>
                                {subState.isOpen && (
                                    <div
                                        className="h-full overflow-y-auto"
                                        style={{ width: `${SUB_DRAWER_WIDTH}px` }}
                                    >
                                        <p className="mb-2">業種: IT・通信</p>
                                        <p className="mb-2">従業員数: 1000人以上</p>
                                    </div>
                                )}
                            </div>
                        </SheetContent>
                    </Sheet>
                </div>
            </div>
        </div>
    );
};

CompanyDrawer.displayName = "CompanyDrawer"

