"use client"

import React, { useMemo, useRef, useCallback } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { ProjectDrawerToolbar } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerToolbar"
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import useSWR from "swr";
import { Skeleton } from "@/components/ui/skeleton"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ProjectStatus } from "@/types/project"

// ステータスに応じた色を定義
const statusColors = {
    [ProjectStatus.Planning]: "bg-blue-100 text-blue-800 hover:bg-blue-100 hover:text-blue-800",
    [ProjectStatus.InProgress]: "bg-yellow-100 text-yellow-800 hover:bg-yellow-100 hover:text-yellow-800",
    [ProjectStatus.Completed]: "bg-green-100 text-green-800 hover:bg-green-100 hover:text-green-800",
    [ProjectStatus.OnHold]: "bg-gray-100 text-gray-800 hover:bg-gray-100 hover:text-gray-800",
    [ProjectStatus.Cancelled]: "bg-red-100 text-red-800 hover:bg-red-100 hover:text-red-800",
};

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

    const { project, isLoading, error } = useProjectDetails(selectedProjectId);

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
                {project && <ProjectDetails project={project} />}
            </div>
        </div>
    )
});

const LoadingSkeleton: React.FC = () => (
    <div className="space-y-4">
        <Skeleton className="h-8 w-3/4" />
        <Skeleton className="h-4 w-1/2" />
        <Skeleton className="h-4 w-2/3" />
        <Skeleton className="h-4 w-1/3" />
    </div>
);

const ErrorAlert: React.FC<{ error: Error }> = ({ error }) => (
    <Alert variant="destructive">
        <AlertTitle>エラーが発生しました</AlertTitle>
        <AlertDescription>{error.message}</AlertDescription>
    </Alert>
);

const ProjectDetails: React.FC<{ project: any }> = ({ project }) => (
    <Card>
        <CardHeader className="flex justify-between items-center">
            <CardTitle>{project.title}</CardTitle>
            <Badge className={statusColors[project.status as keyof typeof statusColors]}>
                {project.status}
            </Badge>
        </CardHeader>
        <CardContent>
            <dl className="grid grid-cols-2 gap-4">
                <div className="col-span-2">
                    <dt className="font-semibold">技術スタック</dt>
                    <dd>{project.skill_labels?.join(', ')}</dd>
                </div>
                <div className="col-span-2">
                    <dt className="font-semibold">内容</dt>
                    <dd>{project.description}</dd>
                </div>
                <div>
                    <dt className="font-semibold">時給</dt>
                    <dd>{project.hourly_pay}円</dd>
                </div>
                <div>
                    <dt className="font-semibold">総作業時間</dt>
                    <dd>{project.total_working_time}時間</dd>
                </div>
            </dl>
        </CardContent>
    </Card>
);

ProjectDrawerBody.displayName = "ProjectDrawerBody"