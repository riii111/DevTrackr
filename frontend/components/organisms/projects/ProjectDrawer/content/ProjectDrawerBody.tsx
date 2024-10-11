"use client"

import React, { useMemo, useRef, useCallback, useState } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { ProjectDrawerToolbar } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerToolbar"
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import useSWR from "swr";
import { ErrorAlert } from "@/components/organisms/projects/ProjectDrawer/content/ErrorAlert"
import { ProjectDetails } from "@/components/organisms/projects/ProjectDrawer/ProjectDetails"
import { LoadingSkeleton } from "@/components/organisms/projects/ProjectDrawer/content/LoadingSkeleton"

interface Props {
    width?: number
    drawerType: "main" | "sub"
    selectedProjectId?: string | null
}

function useProjectDetails(projectId: string | null) {
    const { getProjectById } = useProjectsApi();

    const fetchProject = useCallback(() => {
        return projectId ? getProjectById(projectId) : null;
    }, [projectId, getProjectById]);

    const { data, error, isLoading } = useSWR(
        projectId ? `project-${projectId}` : null,
        fetchProject,
        { revalidateOnFocus: false } // 閉じる際にも取得されるのを防ぐ
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

    const { project, isLoading, error } = useProjectDetails(selectedProjectId ?? null);

    const handleSave = (updatedProject: any) => {
        // TODO: PUTリクエストを送信
        console.log("Updated project:", updatedProject);
    };

    return (
        <div
            ref={isSubDrawer ? subDrawer : undefined}
            style={drawerStyle}
            className={`flex flex-col min-h-screen ${isSubDrawer ? 'shadow-inner transition-all duration-300' : ''}`}
        >
            <ProjectDrawerToolbar drawerType={drawerType} />
            <hr className="border-gray-300" />
            <div className="p-4">
                {isLoading && <LoadingSkeleton />}
                {error && <ErrorAlert error={error} />}
                {project && <ProjectDetails project={project} onSave={handleSave} />}
            </div>
        </div>
    )
});

ProjectDrawerBody.displayName = "ProjectDrawerBody"