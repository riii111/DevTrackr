"use client"

import { useMemo, useRef } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"



interface Props {
    width?: number
    drawerType: "main" | "sub"
}

export default function ProjectDrawerBody({ width, drawerType }: Props) {
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
            className={`d-flex flex-column min-h-100 ${isSubDrawer ? 'inner-shadow sub-drawer-transition' : ''}`}
        >
            <hr className="border-gray-300" />
            <span> ここにProjectDrawerNameが入ります</span>
            {/* <ProjectDrawerName drawerType={drawerType} /> */}
            <hr className="border-gray-300" />
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
}