'use client';

import React, { useState, useEffect, useCallback, useRef } from 'react';
import Draggable from 'react-draggable';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Timer, Play, Square, GripHorizontal } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { useWorkLog } from '@/lib/store/useWorkLogStore';
import { getProjectById } from "@/lib/api/projects";
import { ProjectResponse } from "@/types/project";

export function WorkLogDialog() {
    const { state, dispatch } = useWorkLog();
    const [project, setProject] = useState<ProjectResponse | null>(null);

    const [startTime, setStartTime] = useState<Date | null>(null);
    const [endTime, setEndTime] = useState<Date | null>(null);
    const [memo, setMemo] = useState("");
    const dialogRef = useRef<HTMLDivElement>(null);
    const prevProjectId = useRef<string | null>(null);

    const fetchProject = useCallback(async (projectId: string) => {
        try {
            const fetchedProject = await getProjectById(projectId);
            setProject(fetchedProject);
        } catch (error) {
            console.error("プロジェクトの取得に失敗しました:", error);
        }
    }, []);

    useEffect(() => {
        if (state.isOpen && state.projectId && state.projectId !== prevProjectId.current) {
            fetchProject(state.projectId);
            prevProjectId.current = state.projectId;
        }
    }, [state.isOpen, state.projectId, fetchProject]);

    const handleClose = useCallback(() => {
        dispatch({ type: 'CLOSE_WORK_LOG' });
        setStartTime(null);
        setEndTime(null);
        setMemo("");
        setProject(null);
        prevProjectId.current = null;
    }, [dispatch]);

    const handleStartWork = () => {
        setStartTime(new Date());
        setEndTime(null);
    };

    const handleEndWork = () => {
        setEndTime(new Date());
    };

    const handleSubmit = useCallback((e: React.FormEvent) => {
        e.preventDefault();
        console.log("稼働記録:", { project, startTime, endTime, memo });
        handleClose();
    }, [project, startTime, endTime, memo, handleClose]);

    if (!state.isOpen) return null;

    return (
        <Dialog open={state.isOpen}>
            <Draggable
                handle=".drag-handle"
                bounds="parent"
                defaultPosition={{ x: 0, y: 0 }}
                positionOffset={{ x: '50%', y: '25%' }}
            >
                <DialogContent
                    ref={dialogRef}
                    className="w-96 max-w-full bg-white shadow-lg rounded-lg overflow-hidden"
                    style={{
                        position: 'fixed',
                        margin: 0,
                        transform: 'none',
                        pointerEvents: 'auto',
                        zIndex: 50
                    }}
                    noOverlay={true}
                >
                    <DialogHeader className="space-y-4 pb-4 border-b">
                        <div className="flex justify-between items-center cursor-grab drag-handle select-none">
                            <GripHorizontal className="h-5 w-5 text-gray-400" />
                            <DialogTitle className="text-lg font-bold flex-1 text-center">
                                稼働記録
                                {project && (
                                    <Badge variant="secondary" className="ml-2">
                                        {project.title}
                                    </Badge>
                                )}
                            </DialogTitle>
                            <Button
                                variant="ghost"
                                size="icon"
                                onClick={handleClose}
                                className="rounded-full hover:bg-gray-100"
                            >
                                <Timer className="h-4 w-4" />
                            </Button>
                        </div>
                    </DialogHeader>

                    {project ? (
                        <form onSubmit={handleSubmit} className="space-y-4 p-4">
                            <Card className="bg-gray-50 p-4">
                                <div className="flex justify-center space-x-4">
                                    <Button
                                        type="button"
                                        onClick={handleStartWork}
                                        disabled={!!startTime}
                                        className="w-full"
                                        variant={startTime ? "secondary" : "default"}
                                    >
                                        <Play className="mr-2 h-4 w-4" />
                                        開始
                                    </Button>
                                    <Button
                                        type="button"
                                        onClick={handleEndWork}
                                        disabled={!startTime || !!endTime}
                                        className="w-full"
                                        variant={endTime ? "secondary" : "default"}
                                    >
                                        <Square className="mr-2 h-4 w-4" />
                                        終了
                                    </Button>
                                </div>
                            </Card>

                            <div className="space-y-2 text-sm">
                                {startTime && (
                                    <div className="text-gray-600">
                                        開始: {startTime.toLocaleString()}
                                    </div>
                                )}
                                {endTime && (
                                    <div className="text-gray-600">
                                        終了: {endTime.toLocaleString()}
                                    </div>
                                )}
                            </div>

                            <div className="space-y-2">
                                <label htmlFor="memo" className="text-sm font-medium block">
                                    メモ
                                </label>
                                <Textarea
                                    id="memo"
                                    value={memo}
                                    onChange={(e) => setMemo(e.target.value)}
                                    placeholder="作業内容を入力してください"
                                    className="h-24 resize-none"
                                />
                            </div>

                            <Button
                                type="submit"
                                disabled={!startTime || !endTime}
                                className="w-full"
                            >
                                記録を保存
                            </Button>
                        </form>
                    ) : (
                        <div className="flex items-center justify-center h-32">
                            <p className="text-sm text-gray-500">
                                プロジェクト情報を読み込み中...
                            </p>
                        </div>
                    )}
                </DialogContent>
            </Draggable>
        </Dialog>
    );
}

export default WorkLogDialog;
