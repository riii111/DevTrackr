"use client"

import React, { useMemo, useRef } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { ProjectDrawerToolbar } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerToolbar"
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import useSWR from "swr";

interface Props {
    width?: number
    drawerType: "main" | "sub"
    selectedProjectId?: string | null
}


function useProjectDetails(projectId: string | null) {
    const { getProjectById } = useProjectsApi();

    const { data, error, isLoading } = useSWR(
        projectId ? `project-${projectId}` : null,
        () => projectId ? getProjectById(projectId) : null
    );

    return {
        project: data,
        isLoading,
        error
    };
}


export const ProjectDrawerBody: React.FC<Props> = React.memo(({ width, drawerType, selectedProjectId }) => {
    const drawerStore = useDrawerStore()
    const subDrawer = useRef<HTMLDivElement>(null)


    const state = drawerStore.drawerState[drawerType]
    const isSubDrawer = drawerType == "sub"

    const drawerStyle = useMemo(() => {
        if (isSubDrawer) {
            return {
                width: `${state.isOpen ? width : 0}px`
            }
        }
        return undefined
    }, [isSubDrawer, state.isOpen, width])

    console.log("called ProjectDrawerBody")

    const { project, isLoading, error } = useProjectDetails(selectedProjectId);

    return (
        <div
            ref={isSubDrawer ? subDrawer : undefined}
            style={drawerStyle}
            className={`flex flex-col min-h-screen ${isSubDrawer ? 'shadow-inner transition-all duration-300' : ''}`}>
            <ProjectDrawerToolbar
                drawerType={drawerType}
            />
            <hr className="border-gray-300" />
            {isLoading && <p>読み込み中...</p>}
            {error && <p>エラーが発生しました: {error.message}</p>}
            {project && (
                <div>
                    <p>プロジェクトID: {project.id}</p>
                    <p>プロジェクト名: {project.title}</p>
                    <p>技術スタック: {project.skill_labels?.join(', ')}</p>
                    <p>内容: {project.description}</p>
                    <p>時給: {project.hourly_pay}</p>
                    <p>ステータス: {project.status}</p>
                    {/* プロジェクトの他の詳細情報を表示 */}
                </div>
            )}
        </div>
    )
});

ProjectDrawerBody.displayName = "ProjectDrawerBody"