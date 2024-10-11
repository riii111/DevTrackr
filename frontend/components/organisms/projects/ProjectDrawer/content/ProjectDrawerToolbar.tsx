"use client"
import { useCallback, useState, useRef } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { Tooltip, TooltipTrigger, TooltipContent } from "@/components/ui/tooltip"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Dialog, DialogContent, DialogHeader } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { RiMoreLine, RiContractUpDownLine } from 'react-icons/ri'
import { BsArrowsFullscreen } from "react-icons/bs";
import { BsFullscreenExit } from "react-icons/bs";
import { DialogTitle } from "@radix-ui/react-dialog"
import { TooltipProvider } from "@radix-ui/react-tooltip"

interface Props {
    drawerType: "main" | "sub"
}

export function ProjectDrawerToolbar({ drawerType }: Props) {
    const drawerStore = useDrawerStore()
    const state = drawerStore.drawerState[drawerType]
    const firstFocus = useRef<HTMLDivElement>(null) // 

    const [confirmDeleteDialogOpen, setConfirmDeleteDialogOpen] = useState(false)


    const computedDataTypeText = "プロジェクト"

    // TODO: 全画面表示対応
    const handleToggleFullScreen = () => {
        if (drawerType === "main") {
            drawerStore.setIsFullScreen(!drawerStore.isFullScreen)
        }
    }
    const computedFullscreenCondition = !drawerStore.isFullScreen
    const computedFullscreenLabel = computedFullscreenCondition ? '全画面で表示' : '全画面表示を折りたたむ'

    // TODO: 削除機能を実装
    const handleDelete = useCallback(() => {
        drawerStore.handleClose(drawerType)
    }, [drawerStore, drawerType])

    console.log("called ProjectDrawerToolBar")

    return (
        <div className="flex items-center px-4 py-1 gap-4" ref={firstFocus}>
            <span className="text-sm font-medium text-gray-700">{computedDataTypeText}</span>
            <div className="flex-grow" />
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                        <Button
                            variant="ghost"
                            size="icon"
                            className="w-8 h-8 rounded-full"
                            onClick={handleToggleFullScreen}
                        >
                            {computedFullscreenCondition ? (
                                <BsArrowsFullscreen className="w-4 h-4 text-gray-500" />
                            ) : (
                                <BsFullscreenExit className="w-4 h-4 text-gray-500" />
                            )}
                        </Button>
                    </TooltipTrigger>
                    <TooltipContent>{computedFullscreenLabel}</TooltipContent>
                </Tooltip>
            </TooltipProvider>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button
                        variant="ghost"
                        size="icon"
                        className="w-8 h-8 rounded-full"
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