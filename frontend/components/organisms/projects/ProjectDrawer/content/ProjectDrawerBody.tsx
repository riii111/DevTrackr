"use client"

import React, { useMemo, useRef } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { ProjectDrawerToolbar } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawerToolbar"


interface Props {
    width?: number
    drawerType: "main" | "sub"
    selectedProjectId?: string | null
}

export const ProjectDrawerBody: React.FC<Props> = React.memo(({ width, drawerType, selectedProjectId }) => {
    const drawerStore = useDrawerStore()
    // const projectStore = useProjectStore()
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

    return (
        <div
            ref={isSubDrawer ? subDrawer : undefined}
            style={drawerStyle}
            className={`flex flex-col min-h-screen ${isSubDrawer ? 'shadow-inner transition-all duration-300' : ''}`}>
            <ProjectDrawerToolbar
                drawerType={drawerType}
            />
            <hr className="border-gray-300" />
            <span> ここにProjectDrawerNameが入ります</span>
            {selectedProjectId && (
                <div>
                    <p>プロジェクトID: {selectedProjectId}</p>
                    <p>プロジェクト名: 駐車場管理システム</p>
                    <p>技術スタック: Remix, FastAPI(MongoDB), CloudFlare</p>
                </div>
                // ) : (
                //     <div className="p-4">プロジェクト情報が見つかりません。</div>
            )}
            {/* <ProjectDrawerName drawerType={drawerType} /> */}
            {/* <hr className="border-gray-300" /> */}
            {/* {state.type === 'project' && (
                <ProjectDrawerContentEvent
                    key={`event-${state.id}`}
                    drawerType={drawerType}
                    event={projectStore.getItemByIdAndType({ type: 'event', id: state.id })}
                />
            )}
            {state.type === 'task' && (
                <ProjectDrawerContentTask
                    key={`task-${state.id}`}
                    drawerType={drawerType}
                    task={projectStore.getItemByIdAndType({ type: 'task', id: state.id })}
                />
            )}
            {state.type === 'todo' && (
                <ProjectDrawerContentTodo
                    key={`todo-${state.id}`}
                    todo={projectStore.getItemByIdAndType({ type: 'todo', id: state.id })}
                />
            )} */}
        </div>
    )
});

ProjectDrawerBody.displayName = "ProjectDrawerBody"