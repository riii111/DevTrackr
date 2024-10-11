"use client";

import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription } from "@/components/ui/sheet"
import { useSearchParams, useRouter } from "next/navigation";
import React, { useEffect, useState, useCallback, useMemo } from "react";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { ProjectDrawerBody } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerBody"

const DRAWER_WIDTH = 640
const MAIN_DRAWER_FULL_MIN_WIDTH = 1000
const SUB_DRAWER_WIDTH = 520

const windowResizeObserver = (setWindowWidth: React.Dispatch<React.SetStateAction<number>>) => {
    setWindowWidth(window.innerWidth);
};

export const ProjectDrawer = React.memo(() => {
    const router = useRouter();
    const searchParams = useSearchParams();
    const drawerStore = useDrawerStore();
    const mainState = drawerStore.drawerState.main
    const subState = drawerStore.drawerState.sub
    const [windowWidth, setWindowWidth] = useState(0)

    const selectedProjectId = searchParams.get("projectId");

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

    useEffect(() => {
        if (selectedProjectId) {
            fetchProjectDetails(selectedProjectId);
        }
    }, [selectedProjectId]);

    const fetchProjectDetails = async (projectId: string) => {
        try {
            // const projectData = await api.getProjectDetails(projectId);
            // ここでプロジェクト詳細データを設定する
        } catch (error) {
            console.error('プロジェクト詳細の取得に失敗しました:', error);
            // エラー状態を設定するなどの処理を追加
        }
    };

    const onUpdateModelValue = (isOpen: boolean) => {
        if (!isOpen) {
            drawerStore.handleClose("main");
            router.push("/dashboard/projects");
        }
    };

    console.log("called ProjectDrawer")

    return (
        <div className="fixed inset-0 overflow-hidden z-50">
            <div className="absolute inset-0 overflow-hidden">
                <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                    <Sheet open={mainState.isOpen} onOpenChange={onUpdateModelValue}>
                        <SheetContent
                            side="right"
                            className="p-0 w-full sm:max-w-full bg-dialog-bg text-primary"
                            style={{ width: containerWidth }}
                        >
                            <SheetHeader>
                                <SheetTitle className="text-primary font-bold">プロジェクト詳細</SheetTitle>
                                <SheetDescription>プロジェクトの詳細情報を表示する</SheetDescription>
                            </SheetHeader>
                            <div className="flex h-screen">
                                <div
                                    className="h-full overflow-y-auto"
                                    style={{ width: `${mainDrawerWidth}px` }}
                                >
                                    <ProjectDrawerBody drawerType="main" selectedProjectId={selectedProjectId} />
                                </div>
                                {subState.isOpen && (
                                    <div
                                        className="h-full overflow-y-auto"
                                        style={{ width: `${SUB_DRAWER_WIDTH}px` }}
                                    >
                                        {/* <ProjectDrawerBody drawerType="sub" width={SUB_DRAWER_WIDTH} /> */}
                                        <p className="mb-2">開発フェーズ: DB設計中</p>
                                        <p className="mb-2">協力会社: ナントカ株式会社、ふんちゃら株式会社...</p>
                                    </div>
                                )}
                            </div>
                        </SheetContent>
                    </Sheet>
                </div>
            </div>
        </div>
    );
});

ProjectDrawer.displayName = "ProjectDrawer"