"use client";

// import { BaseDrawer } from "@/components/organisms/projects/ProjectDrawer/BaseDrawer";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription } from "@/components/ui/sheet"
import { useSearchParams, useRouter } from "next/navigation";
import React, { useEffect, useState, useCallback, useMemo } from "react";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { ProjectDrawerBody } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerBody"

const DRAWER_WIDTH = 640
const MAIN_DRAWER_FULL_MIN_WIDTH = 1000
const SUB_DRAWER_WIDTH = 520

export const ProjectDrawer = React.memo(() => {
    // export function ProjectDrawer() {
    const router = useRouter();
    const searchParams = useSearchParams();
    const drawerStore = useDrawerStore();
    const mainState = drawerStore.drawerState.main
    const subState = drawerStore.drawerState.sub
    const [windowWidth, setWindowWidth] = useState(0)

    const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);

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


    const windowResizeObserver = useCallback(() => {
        setWindowWidth(window.innerWidth)
    }, [])

    // TODO: FullScreenとクエリパラメータ対応時にObserver実装する
    useEffect(() => {
        windowResizeObserver();
        window.addEventListener('resize', windowResizeObserver);

        return () => {
            window.removeEventListener('resize', windowResizeObserver);
        };
    }, [windowResizeObserver]);

    useEffect(() => {
        const projectId = searchParams.get("projectId");
        if (projectId) {
            // ここでプロジェクトIDを使用してプロジェクト情報を取得する
            // 例: APIリクエストを送信してプロジェクト詳細を取得
            setSelectedProjectId(projectId)
            // fetchProjectDetails(projectId);
        }
    }, [searchParams]);

    // const fetchProjectDetails = async (projectId: string) => {
    // ここでAPIリクエストを実装し、プロジェクト詳細を取得
    // 取得したデータでsetSelectedProjectを呼び出す
    // 例: const projectData = await api.getProjectDetails(projectId);
    // setSelectedProject(projectData);
    // };

    const onUpdateModelValue = useCallback((isOpen: boolean) => {
        if (!isOpen) {
            drawerStore.handleClose("main");
            router.push("/dashboard/time-tracking");
        }
    }, [drawerStore, router])

    console.log("called ProjectDrawer")

    return (
        <div className="fixed inset-0 overflow-hidden z-50">
            <div className="absolute inset-0 overflow-hidden">
                <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                    <Sheet open={mainState.isOpen} onOpenChange={onUpdateModelValue}>
                        <SheetContent
                            side="right"
                            className="p-0 w-full sm:max-w-full"
                            style={{ width: containerWidth }}
                        >
                            <SheetHeader>
                                <SheetTitle>プロジェクト詳細</SheetTitle>
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
                                        <p className="mb-2">協力会社: Sky株式会社、...</p>
                                    </div>
                                )}
                            </div>
                        </SheetContent>
                    </Sheet>

                    {/* <BaseDrawer
                        isOpen={drawerStore.drawerState.main.isOpen}
                        onOpenChange={(open) => {
                            if (!open) {
                                drawerStore.handleClose("main");
                                router.push("/dashboard/time-tracking");
                            }
                        }}
                        title="プロジェクト詳細"
                    >
                        {selectedProject ? (
                            <div className="p-4">
                                <p className="mb-2">プロジェクトID: {selectedProject.id}</p>
                                <p className="mb-2">プロジェクト名: {selectedProject.name}</p>
                            </div>
                        ) : (
                            <div className="p-4">プロジェクト情報が見つかりません。</div>
                        )}
                    </BaseDrawer> */}
                </div>
            </div>
        </div>
    );
});

ProjectDrawer.displayName = "ProjectDrawer"