"use client";

import { BaseDrawer } from "@/components/organisms/projects/ProjectDrawer/BaseDrawer";
import { useSearchParams, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useDrawerStore } from "@/lib/store/useDrawerStore";

interface Project {
    id: string;
    name: string;
    // 他のプロジェクト関連のプロパティを追加
}

export function ProjectDrawer() {
    const searchParams = useSearchParams();
    const [selectedProject, setSelectedProject] = useState<Project | null>(null);
    const drawerStore = useDrawerStore();
    const router = useRouter();
    useEffect(() => {
        const projectId = searchParams.get("projectId");
        if (projectId) {
            // ここでプロジェクトIDを使用してプロジェクト情報を取得する
            // 例: APIリクエストを送信してプロジェクト詳細を取得
            fetchProjectDetails(projectId);
        }
    }, [searchParams]);

    const fetchProjectDetails = async (projectId: string) => {
        // ここでAPIリクエストを実装し、プロジェクト詳細を取得
        // 取得したデータでsetSelectedProjectを呼び出す
        // 例: const projectData = await api.getProjectDetails(projectId);
        // setSelectedProject(projectData);
    };

    return (
        <div className="fixed inset-0 overflow-hidden z-50">
            <div className="absolute inset-0 overflow-hidden">
                <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10">
                    <BaseDrawer
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
                                {/* 他のプロジェクト詳細情報を追加 */}
                            </div>
                        ) : (
                            <div className="p-4">プロジェクト情報が見つかりません。</div>
                        )}
                    </BaseDrawer>
                </div>
            </div>
        </div>
    );
}