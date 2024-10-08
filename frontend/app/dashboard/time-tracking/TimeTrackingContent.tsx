"use client";

import { ProjectList } from "@/components/organisms/projects/ProjectList/ProjectList";
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";

export default function TimeTrackingContent({ bgColor }: { bgColor: string }) {
    const { projects, isLoading, isError } = useProjectsApi();
    return (
        <div className={`p-6 rounded-lg ${bgColor} text-text-primary`}>
            <h1 className="text-2xl font-bold mb-4">勤怠</h1>
            {isLoading ? (
                <p>プロジェクト一覧を読み込み中...</p>
            ) : isError ? (
                <p>プロジェクト一覧の取得に失敗しました</p>
            ) : (
                <ProjectList projects={projects} />
            )}
        </div>
    );
}
