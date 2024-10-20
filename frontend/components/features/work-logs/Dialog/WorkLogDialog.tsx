'use client';

import { useWorkLog } from '@/lib/store/useWorkLogStore';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useState, useEffect, useCallback, useMemo } from 'react';
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";

export function WorkLogDialog() {
    const { state, dispatch } = useWorkLog();
    const [project, setProject] = useState<Project | null>(null);
    const { getProjectById } = useProjectsApi();

    const [startTime, setStartTime] = useState(() => new Date().toISOString().slice(0, 16));
    const [endTime, setEndTime] = useState(() => new Date(Date.now() + 3600000).toISOString().slice(0, 16));
    const [memo, setMemo] = useState("");

    const fetchProject = useCallback(async (projectId: string) => {
        try {
            const fetchedProject = await getProjectById(projectId);
            console.log("fetchedProject", fetchedProject);
            setProject(fetchedProject);
        } catch (error) {
            console.error("プロジェクトの取得に失敗しました:", error);
        }
    }, [getProjectById]);

    useEffect(() => {
        if (state.isOpen && state.projectId && (!project || project.id !== state.projectId)) {
            fetchProject(state.projectId);
        }
    }, [state.isOpen, state.projectId, project, fetchProject]);

    const handleClose = useCallback(() => dispatch({ type: 'CLOSE_WORK_LOG' }), [dispatch]);

    const handleSubmit = useCallback((e: React.FormEvent) => {
        e.preventDefault();
        console.log("稼働記録:", { project, startTime, endTime, memo });
        handleClose();
    }, [project, startTime, endTime, memo, handleClose]);

    if (!state.isOpen) return null;

    return (
        <Dialog open={true} onOpenChange={handleClose}>
            <DialogContent className="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>稼働記録 {project ? `- ${project.title}` : ''}</DialogTitle>
                </DialogHeader>
                {project ? (
                    <form onSubmit={handleSubmit} className="space-y-4">
                        <div className="space-y-2">
                            <label htmlFor="start-time">開始時間</label>
                            <Input
                                id="start-time"
                                type="datetime-local"
                                value={startTime}
                                onChange={(e) => setStartTime(e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <label htmlFor="end-time">終了時間</label>
                            <Input
                                id="end-time"
                                type="datetime-local"
                                value={endTime}
                                onChange={(e) => setEndTime(e.target.value)}
                            />
                        </div>
                        <div className="space-y-2">
                            <label htmlFor="memo">メモ</label>
                            <Textarea
                                id="memo"
                                value={memo}
                                onChange={(e) => setMemo(e.target.value)}
                                placeholder="作業内容を入力してください"
                            />
                        </div>
                        <div className="flex justify-end space-x-2">
                            <Button type="button" variant="outline" onClick={handleClose}>
                                キャンセル
                            </Button>
                            <Button type="submit">記録する</Button>
                        </div>
                    </form>
                ) : (
                    <p>プロジェクト情報を読み込み中...</p>
                )}
            </DialogContent>
        </Dialog>
    );
}
