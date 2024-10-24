'use client';

import { useWorkLog } from '@/lib/store/useWorkLogStore';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { useState, useEffect, useCallback, useRef } from 'react';
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import { ProjectResponse } from "@/types/project";

export function WorkLogDialog() {
    const { state, dispatch } = useWorkLog();
    const [project, setProject] = useState<ProjectResponse | null>(null);
    const { getProjectById } = useProjectsApi();

    const [startTime, setStartTime] = useState<Date | null>(null);
    const [endTime, setEndTime] = useState<Date | null>(null);
    const [memo, setMemo] = useState("");

    const prevProjectId = useRef<string | null>(null);

    const fetchProject = useCallback(async (projectId: string) => {
        try {
            const fetchedProject = await getProjectById(projectId);
            setProject(fetchedProject);
        } catch (error) {
            console.error("プロジェクトの取得に失敗しました:", error);
        }
    }, [getProjectById]);

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
        <Dialog open={true} onOpenChange={handleClose}>
            <DialogContent noOverlay className="w-96 max-w-full">
                <DialogHeader>
                    <DialogTitle>稼働記録 {project ? `- ${project.title}` : ''}</DialogTitle>
                </DialogHeader>
                {project ? (
                    <form onSubmit={handleSubmit} className="space-y-4">
                        <div className="flex justify-between">
                            <Button type="button" onClick={handleStartWork} disabled={!!startTime}>
                                稼働開始
                            </Button>
                            <Button type="button" onClick={handleEndWork} disabled={!startTime || !!endTime}>
                                稼働終了
                            </Button>
                        </div>
                        {startTime && (
                            <div className="text-sm">
                                開始時間: {startTime.toLocaleString()}
                            </div>
                        )}
                        {endTime && (
                            <div className="text-sm">
                                終了時間: {endTime.toLocaleString()}
                            </div>
                        )}
                        <div className="space-y-2">
                            <label htmlFor="memo" className="text-sm font-medium">メモ</label>
                            <Textarea
                                id="memo"
                                value={memo}
                                onChange={(e) => setMemo(e.target.value)}
                                placeholder="作業内容を入力してください"
                                className="h-24"
                            />
                        </div>
                        <div className="flex justify-end space-x-2">
                            <Button type="button" variant="outline" onClick={handleClose}>
                                キャンセル
                            </Button>
                            <Button type="submit" disabled={!startTime || !endTime}>送信</Button>
                        </div>
                    </form>
                ) : (
                    <p className="text-sm">プロジェクト情報を読み込み中...</p>
                )}
            </DialogContent>
        </Dialog>
    );
}
