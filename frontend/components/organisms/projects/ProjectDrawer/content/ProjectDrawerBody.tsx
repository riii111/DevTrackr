"use client"

import React, { useMemo, useRef, useCallback, useState } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { ProjectDrawerToolbar } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerToolbar"
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import useSWR from "swr";
import { Skeleton } from "@/components/ui/skeleton"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { PencilIcon, CheckIcon, XIcon } from 'lucide-react'
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

    const handleSave = (updatedProject: any) => {
        // ここでプロジェクトの更新処理を実装する
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

interface ProjectDetailsProps {
    project: any;
    onSave: (updatedProject: any) => void;
}

export const ProjectDetails: React.FC<ProjectDetailsProps> = ({ project, onSave }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedProject, setEditedProject] = useState(project);

    const handleEdit = () => {
        setIsEditing(true);
    };

    const handleSave = () => {
        onSave(editedProject);
        setIsEditing(false);
    };

    const handleCancel = () => {
        setEditedProject(project);
        setIsEditing(false);
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setEditedProject(prev => ({ ...prev, [name]: value }));
    };

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-2xl font-bold">
                    {isEditing ? (
                        <Input
                            name="title"
                            value={editedProject.title}
                            onChange={handleInputChange}
                            className="text-2xl font-bold"
                        />
                    ) : (
                        editedProject.title
                    )}
                </CardTitle>
                <Badge className={statusColors[project.status as keyof typeof statusColors]}>
                    {project.status}
                </Badge>
            </CardHeader>
            <CardContent>
                <dl className="space-y-4">
                    <div>
                        <dt className="font-semibold">技術スタック</dt>
                        <dd>
                            {isEditing ? (
                                <Input
                                    name="skill_labels"
                                    value={editedProject.skill_labels?.join(', ')}
                                    onChange={handleInputChange}
                                />
                            ) : (
                                editedProject.skill_labels?.join(', ')
                            )}
                        </dd>
                    </div>
                    <div>
                        <dt className="font-semibold">内容</dt>
                        <dd>
                            {isEditing ? (
                                <Textarea
                                    name="description"
                                    value={editedProject.description}
                                    onChange={handleInputChange}
                                    rows={3}
                                />
                            ) : (
                                editedProject.description
                            )}
                        </dd>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <dt className="font-semibold">時給</dt>
                            <dd>
                                {isEditing ? (
                                    <Input
                                        name="hourly_pay"
                                        type="number"
                                        value={editedProject.hourly_pay}
                                        onChange={handleInputChange}
                                    />
                                ) : (
                                    `${editedProject.hourly_pay}円`
                                )}
                            </dd>
                        </div>
                        <div>
                            <dt className="font-semibold">総作業時間</dt>
                            <dd>
                                {isEditing ? (
                                    <Input
                                        name="total_working_time"
                                        type="number"
                                        value={editedProject.total_working_time}
                                        onChange={handleInputChange}
                                    />
                                ) : (
                                    `${editedProject.total_working_time}時間`
                                )}
                            </dd>
                        </div>
                    </div>
                </dl>
                <div className="mt-4 flex justify-end space-x-2">
                    {isEditing ? (
                        <>
                            <Button onClick={handleSave} variant="default" className="text-white hover:bg-primary/80">
                                <CheckIcon className="mr-2 h-4 w-4 text-white" /> 保存
                            </Button>
                            <Button onClick={handleCancel} variant="outline" className="text-white hover:bg-primary/80">
                                <XIcon className="mr-2 h-4 w-4 text-white" /> キャンセル
                            </Button>
                        </>
                    ) : (
                        <Button onClick={handleEdit} variant="outline" className="text-white hover:bg-primary/80">
                            <PencilIcon className="mr-2 h-4 w-4 text-white" /> 編集
                        </Button>
                    )}
                </div>
            </CardContent>
        </Card>
    );
};

ProjectDrawerBody.displayName = "ProjectDrawerBody"