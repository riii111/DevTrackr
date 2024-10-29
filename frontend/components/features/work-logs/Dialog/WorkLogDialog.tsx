'use client';

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Textarea } from "@/components/ui/textarea";
import { format, isToday } from 'date-fns';
import { ja } from 'date-fns/locale';
import { useWorkLog } from '@/lib/store/useWorkLogStore';
import { getProjectById } from "@/lib/api/projects";
import { ProjectResponse } from "@/types/project";
import { useAutoSave } from '@/lib/hooks/useAutoSave';
import { useDraggable } from '@/lib/hooks/useDraggable';
import { Header } from '@/components/features/work-logs/Dialog/contents/Header';
import { ActionButtons } from '@/components/features/work-logs/Dialog/contents/ActionButton';
import { TimeInfo } from '@/components/features/work-logs/Dialog/contents/TimeInfo';
import '@/components/features/work-logs/Dialog/contents/dialog.css';
import { createWorkLogAction, updateWorkLogAction } from "@/lib/actions/worklog";
import { useToast } from "@/lib/hooks/use-toast";

export function WorkLogDialog() {
    const { state, dispatch } = useWorkLog();
    const dialogRef = useRef<HTMLDivElement>(null);
    const [project, setProject] = useState<ProjectResponse | null>(null);
    const [startTime, setStartTime] = useState<Date | null>(null);
    const [endTime, setEndTime] = useState<Date | null>(null);
    const [memo, setMemo] = useState("");
    const prevProjectId = useRef<string | null>(null);

    const [dialogPosition, setDialogPosition] = useState<{ x: number; y: number } | null>(null);
    const [isVisible, setIsVisible] = useState(false);
    const [breakTime, setBreakTime] = useState(0);
    const [isPaused, setIsPaused] = useState(false);
    const [pauseStartTime, setPauseStartTime] = useState<Date | null>(null);
    const [workLogId, setWorkLogId] = useState<string | null>(null);
    const { toast } = useToast();
    // ドラッグ機能の適用
    const { position, isDragging, handleMouseDown } = useDraggable(
        dialogPosition || { x: 0, y: 0 },
        dialogRef
    );

    const { lastAutoSave, isDirty, isSaving, restoreState } = useAutoSave({
        project_id: project?.id,
        start_time: startTime?.toISOString(),
        end_time: endTime?.toISOString(),
        memo,
        break_time: breakTime,
        workLogId
    });

    // ローカルストレージからの復元を行う。意図しない操作などに対応
    const restoreFromLocalStorage = useCallback((projectId: string) => {
        try {
            const savedData = localStorage.getItem(`workLog_autosave_${projectId}`);
            if (savedData) {
                const parsedData = JSON.parse(savedData);
                const lastModified = new Date(parsedData.lastModified);
                const now = new Date();

                // 30分以内の変更のみ復元
                if ((now.getTime() - lastModified.getTime()) <= 30 * 60 * 1000) {
                    if (parsedData.start_time) setStartTime(new Date(parsedData.start_time));
                    if (parsedData.end_time) setEndTime(new Date(parsedData.end_time));
                    if (parsedData.memo) setMemo(parsedData.memo);
                    if (parsedData.break_time) setBreakTime(parsedData.break_time);

                    toast({
                        description: "未保存の変更を復元しました",
                    });

                    // 保存状態を復元
                    restoreState(lastModified);
                    return true;
                } else {
                    // 30分以上経過している場合はローカルストレージをクリア
                    localStorage.removeItem(`workLog_autosave_${projectId}`);
                }
            }
        } catch (error) {
            console.error("ローカルストレージからの復元に失敗しました:", error);
        }
        return false;
    }, [restoreState, toast]);

    // プロジェクト読み込み時の処理を修正
    useEffect(() => {
        if (!state.isOpen || !state.projectId) return;
        if (state.projectId === prevProjectId.current) return;

        const getProject = async () => {
            try {
                const fetchedProject = await getProjectById(state.projectId!);
                setProject(fetchedProject);
                prevProjectId.current = state.projectId;

                // プロジェクト読み込み後にローカルストレージから復元を試みる
                restoreFromLocalStorage(state.projectId!);
            } catch (error) {
                console.error("プロジェクトの取得に失敗しました:", error);
            }
        };

        getProject();
    }, [state.isOpen, state.projectId, restoreFromLocalStorage]);

    // ダイアログの位置を計算
    const calculateDialogPosition = useCallback((clickPosition: { x: number; y: number }) => {
        const dialogWidth = 384;
        const dialogHeight = 500;
        const padding = 16;
        const buttonOffset = 8;

        let x = clickPosition.x + buttonOffset;
        let y = Math.max(padding, clickPosition.y - (dialogHeight / 2));

        // 画面端での位置調整
        if (x + dialogWidth + padding > window.innerWidth) {
            x = x - dialogWidth - buttonOffset * 2;
        }
        if (y + dialogHeight > window.innerHeight) {
            y = window.innerHeight - dialogHeight - padding;
        }

        return { x, y };
    }, []);

    // ダイアログの表示制御
    useEffect(() => {
        if (!state.isOpen || !state.clickPosition) return;

        const newPosition = calculateDialogPosition(state.clickPosition);

        // 一度完全に非表示にする
        setIsVisible(false);
        setDialogPosition(null);

        // 位置設定とアニメーション開始のタイミングを分離
        requestAnimationFrame(() => {
            setDialogPosition(newPosition);
            requestAnimationFrame(() => {
                setIsVisible(true);
            });
        });

    }, [state.isOpen, state.clickPosition, calculateDialogPosition]);

    // クリーンアップ処理
    const handleClose = useCallback(() => {
        setIsVisible(false);

        // アニメーション完了後にダイアログを閉じる
        setTimeout(() => {
            dispatch({ type: 'CLOSE_WORK_LOG' });
            // 全ての状態をリセット
            setStartTime(null);
            setEndTime(null);
            setMemo("");
            setProject(null);
            setWorkLogId(null);
            prevProjectId.current = null;

            // 位置情報も完全にリセット
            setDialogPosition(null);

            // ローカルストレージのクリーンアップ
            if (project?.id) {
                localStorage.removeItem(`workLog_autosave_${project.id}`);
            }
        }, 200);
    }, [dispatch, project?.id]);

    const handleStartWork = async () => {
        const now = new Date();
        setStartTime(now);
        setEndTime(null);

        if (!project?.id) return;

        try {
            const result = await createWorkLogAction({
                project_id: project.id,
                start_time: now.toISOString(),
                memo: memo,
                break_time: breakTime
            });
            setWorkLogId(result.data.id);

            if (!result.success) {
                throw new Error(result.error);
            }
        } catch (error) {
            console.error('稼働開始の記録に失敗しました:', error);
            // エラー時の状態を戻す
            setStartTime(null);
        }
    };

    const handleEndWork = async () => {
        // 既に終了時刻が設定されている場合は処理を中断
        if (endTime) return;

        const now = new Date();

        // 開始時刻が設定されていない、または無効な場合は処理を中断
        if (!startTime || isNaN(startTime.getTime())) {
            toast({
                variant: "destructive",
                description: "開始時刻が正しく設定されていません",
            });
            return;
        }

        // 開始時刻より前の時刻は設定不可
        if (now < startTime) {
            toast({
                variant: "destructive",
                description: "終了時刻は開始時刻より後に設定してください",
            });
            return;
        }

        if (!project?.id || !workLogId) return;

        try {
            const workTime = calculateWorkTime();
            const result = await updateWorkLogAction(workLogId, {
                project_id: project.id,
                start_time: startTime.toISOString(),
                end_time: now.toISOString(),
                memo: memo,
                break_time: breakTime,
                actual_work_minutes: workTime ? workTime.totalMinutes : undefined
            });

            if (!result.success) {
                throw new Error(result.error);
            }

            setEndTime(now);
        } catch (error) {
            console.error('稼働終了の記録に失敗しました:', error);
            toast({
                variant: "destructive",
                description: "稼働終了の記録に失敗しました",
            });
        }
    };

    const handleTimeEdit = async (type: 'start' | 'end', newDateTime: Date, isValid: boolean) => {
        if (!isValid || !project?.id || !workLogId) return;

        try {
            // 状態のみ更新（APIの呼び出しはuseAutoSaveに任せる）
            if (type === 'start') {
                setStartTime(newDateTime);
            } else {
                setEndTime(newDateTime);
            }

            toast({
                description: "時刻を更新しました",
            });
        } catch (error) {
            console.error('時刻の更新に失敗しました:', error);
            // エラー時は状態を元に戻す
            if (type === 'start') {
                setStartTime(startTime);
            } else {
                setEndTime(endTime);
            }
            toast({
                variant: "destructive",
                description: "時刻の更新に失敗しました",
            });
        }
    };

    const handlePause = () => {
        if (!isPaused) {
            setPauseStartTime(new Date());
            setIsPaused(true);
        } else {
            const pauseDuration = Math.floor((new Date().getTime() - pauseStartTime!.getTime()) / 60000);
            setBreakTime(prev => prev + pauseDuration);
            setPauseStartTime(null);
            setIsPaused(false);
        }
    };

    const calculateWorkTime = useCallback(() => {
        if (!startTime || !endTime) return null;
        const totalMinutes = Math.floor((endTime.getTime() - startTime.getTime()) / 60000);
        const actualMinutes = totalMinutes - breakTime;
        const hours = Math.floor(actualMinutes / 60);
        const minutes = actualMinutes % 60;
        return { hours, minutes, totalMinutes: actualMinutes };
    }, [startTime, endTime, breakTime]);

    const [currentDate, setCurrentDate] = useState<Date>(new Date());

    useEffect(() => {
        if (state.isOpen) {
            const now = new Date();
            setCurrentDate(now);
        }
    }, [state.isOpen]);

    if (!state.isOpen || !dialogPosition) return null;

    return (
        <div
            ref={dialogRef}
            className={`
                fixed bg-white shadow-xl rounded-lg overflow-hidden border border-gray-200
                transform-gpu
                ${isVisible ? 'scale-100 opacity-100' : 'scale-95 opacity-0'}
            `}
            style={{
                top: `${position.y}px`,
                left: `${position.x}px`,
                width: '384px',
                zIndex: 50,
                transition: isDragging ? 'none' : 'transform 0.2s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.2s cubic-bezier(0.16, 1, 0.3, 1)',
                visibility: isVisible && dialogPosition ? 'visible' : 'hidden'
            }}
            onMouseDown={handleMouseDown}
        >
            <Header project={project} onClose={handleClose} />

            {project ? (
                <div className="space-y-4 p-4 animate-fade-in">
                    <div className="text-center text-sm text-gray-600 mb-2">
                        {format(currentDate, 'yyyy年M月d日（E）', { locale: ja })}の稼働記録
                    </div>

                    <ActionButtons
                        startTime={startTime}
                        endTime={endTime}
                        isPaused={isPaused}
                        onStart={handleStartWork}
                        onEnd={handleEndWork}
                        onPause={handlePause}
                        isToday={isToday(currentDate)}
                    />

                    <TimeInfo
                        startTime={startTime}
                        endTime={endTime}
                        breakTime={breakTime}
                        workTime={calculateWorkTime()}
                        onTimeEdit={handleTimeEdit}
                    />

                    <div className="space-y-2">
                        <Textarea
                            id="memo"
                            value={memo}
                            onChange={(e) => setMemo(e.target.value)}
                            placeholder="作業内容を入力してください"
                            className="h-24 resize-none text-primary"
                        />
                    </div>

                    {lastAutoSave && (
                        <div className="text-xs text-gray-400 text-center">
                            {isSaving ? (
                                "保存中..."
                            ) : isDirty ? (
                                '未保存の変更があります'
                            ) : (
                                `最終保存: ${format(lastAutoSave, 'HH:mm')}`
                            )}
                        </div>
                    )}
                </div>
            ) : (
                <div className="flex items-center justify-center h-32 animate-fade-in">
                    <p className="text-sm text-gray-500">
                        プロジェクト情報を読み込み中...
                    </p>
                </div>
            )}
        </div>
    );
}

export default WorkLogDialog;
