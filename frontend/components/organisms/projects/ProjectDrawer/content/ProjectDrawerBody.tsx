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
            )}
        </div>
    )
});

ProjectDrawerBody.displayName = "ProjectDrawerBody"