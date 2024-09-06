"use client"
import { useMemo, useCallback, useState } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui/tooltip"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Dialog, DialogContent, DialogHeader } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { RiCloseLine, RiMoreLine, RiArrowLeftLine, RiContractUpDownLine } from 'react-icons/ri'
import { DialogTitle } from "@radix-ui/react-dialog"

interface Props {
    drawerType: DrawerType
}

export function ProjectDrawerToolbar({ drawerType }: Props) {
    const drawerStore = useDrawerStore()
    const state = drawerStore.drawerState[drawerType]

    const [confirmDeleteDialogOpen, setConfirmDeleteDialogOpen] = useState(false)

    // const data = useMemo(() => {
    //     if (!state.id || !state.type) return undefined
    //     return projectStore.getItemByIdAndType({
    //         id: state.id,
    //         type: state.type
    //     })
    // }, [state.id, state.type, projectStore])


    const computedDataTypeText = useMemo(() => {
        switch (state.type) {
            case 'event':
                return 'イベント'
            case 'task':
                return 'タスク'
            case 'todo':
                return 'ToDo'
            default:
                return ''
        }
    }, [state.type])

    // TODO: 全画面表示対応
    //   const handleToggleFullScreen = () = {
    //     if (drawerType === "main") {
    //         drawerStore.setIsFullScreen(!drawerStore.isFullScreen)
    //         drawerStore.closeSubDrawer()
    //     } else {
    //         const { id: subId, type: subType } = drawerStore.drawerState.sub
    //         drawerStore.closeSubDrawer()
    //         drawerStore.openMainDrawer(subid, subType)
    //     }
    //   }
    // const computedFullscreenCondition = drawerType === 'sub' || !drawerStore.isFullScreen
    const computedFullscreenCondition = true
    const computedFullscreenLabel = computedFullscreenCondition ? '全画面で表示' : '全画面表示を折りたたむ'

    // TODO: 削除機能を実装
    const handleDelete = useCallback(() => {
        drawerStore.handleClose(drawerType)
    }, [drawerStore, drawerType])

    return (
        <div className="flex items-center px-4 py-1 gap-4">
            <span className="text-sm font-medium text-gray-700">{computedDataTypeText}</span>
            <div className="flex-grow" />
            <Tooltip>
                <TooltipTrigger asChild>
                    <Button
                        variant="ghost"
                        size="icon"
                        className="w-8 h-8 rounded-full"
                        // onClick={handleToggleFullScreen}
                        tabIndex={-1}
                    >
                        {computedFullscreenCondition ? (
                            <RiArrowLeftLine className="w-4 h-4 text-gray-500" />
                        ) : (
                            <RiContractUpDownLine className="w-4 h-4 text-gray-500" />
                        )}
                    </Button>
                </TooltipTrigger>
                <TooltipContent>{computedFullscreenLabel}</TooltipContent>
            </Tooltip>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button
                        variant="ghost"
                        size="icon"
                        className="w-8 h-8 rounded-full"
                        tabIndex={-1}
                    >
                        <RiMoreLine className="w-4 h-4 text-gray-500" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuItem
                        onClick={() => setConfirmDeleteDialogOpen(true)}
                        className="text-red-600"
                    >
                        プロジェクトを削除
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
            <Tooltip>
                <TooltipTrigger asChild>
                    <Button
                        variant="ghost"
                        size="icon"
                        className="w-8 h-8 rounded-full"
                        onClick={() => drawerStore.handleClose(drawerType)}
                        tabIndex={-1}
                    >
                        <RiCloseLine className="w-4 h-4 text-gray-500" />
                    </Button>
                </TooltipTrigger>
                <TooltipContent>閉じる</TooltipContent>
            </Tooltip>

            <Dialog open={confirmDeleteDialogOpen} onOpenChange={setConfirmDeleteDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>プロジェクトを削除</DialogTitle>
                    </DialogHeader>
                    <Button onClick={handleDelete}>削除</Button>
                </DialogContent>
            </Dialog>
        </div>
    )
}