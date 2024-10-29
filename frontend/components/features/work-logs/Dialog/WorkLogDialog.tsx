'use client';

import React, { useEffect, useCallback, useRef, useMemo } from 'react';
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
import { DIALOG_DIMENSIONS, AUTO_SAVE_EXPIRATION } from "@/lib/constants/WorkLogDialog";

// 作業時間の検証
const validateWorkLogTimes = (startTime: Date | null, endTime: Date) => {
    if (!startTime || isNaN(startTime.getTime())) {
        return {
            isValid: false,
            error: "開始時刻が正しく設定されていません"
        };
    }

    if (endTime < startTime) {
        return {
            isValid: false,
            error: "終了時刻は開始時刻より後に設定してください"
        };
    }

    return { isValid: true, error: null };
};

// 作業時間の計算
const calculateWorkTime = (startTime: Date | null, endTime: Date | null, breakTime: number) => {
    if (!startTime || !endTime) return null;
    const totalMinutes = Math.floor((endTime.getTime() - startTime.getTime()) / 60 * 1000);
    const actualMinutes = totalMinutes - breakTime;
    return {
        hours: Math.floor(actualMinutes / 60),
        minutes: actualMinutes % 60,
        totalMinutes: actualMinutes
    };
};

// ダイアログの位置を計算
const calculateDialogPosition = (clickPosition: { x: number; y: number }) => {
    let x = clickPosition.x + DIALOG_DIMENSIONS.buttonOffset;
    let y = Math.max(DIALOG_DIMENSIONS.padding, clickPosition.y - (DIALOG_DIMENSIONS.height / 2));

    // 画面端での位置調整
    if (x + DIALOG_DIMENSIONS.width + DIALOG_DIMENSIONS.padding > window.innerWidth) {
        x = x - DIALOG_DIMENSIONS.width - DIALOG_DIMENSIONS.buttonOffset * 2;
    }
    if (y + DIALOG_DIMENSIONS.height > window.innerHeight) {
        y = window.innerHeight - DIALOG_DIMENSIONS.height - DIALOG_DIMENSIONS.padding;
    }

    return { x, y };
};

export function WorkLogDialog() {
    const {
        state,
        setStartTime,
        setEndTime,
        setMemo,
        setBreakTime,
        setPauseStatus,
        setWorkLogId,
        resetWorkLog,
        closeWorkLog
    } = useWorkLog();

    const dialogRef = useRef<HTMLDivElement>(null);
    const [project, setProject] = React.useState<ProjectResponse | null>(null);
    const [isVisible, setIsVisible] = React.useState(false);
    const [dialogPosition, setDialogPosition] = React.useState<{ x: number; y: number } | null>(null);
    const prevProjectId = useRef<string | null>(null);
    const { toast } = useToast();

    // ドラッグ機能の適用
    const { position, isDragging, handleMouseDown } = useDraggable(
        dialogPosition || { x: 0, y: 0 },
        dialogRef
    );

    const autoSaveProps = useMemo(() => ({
        project_id: project?.id,
        start_time: state.startTime?.toISOString(),
        end_time: state.endTime?.toISOString(),
        memo: state.memo,
        break_time: state.breakTime,
        workLogId: state.workLogId
    }), [
        project?.id,
        state.startTime,
        state.endTime,
        state.memo,
        state.breakTime,
        state.workLogId
    ]);

    const { lastAutoSave, isDirty, isSaving, restoreState } = useAutoSave(autoSaveProps);

    // ローカルストレージからの復元を行う。意図しない操作などに対応
    const restoreFromLocalStorage = useCallback((projectId: string) => {
        try {
            const savedData = localStorage.getItem(`workLog_autosave_${projectId}`);
            if (savedData) {
                const parsedData = JSON.parse(savedData);
                const lastModified = new Date(parsedData.lastModified);
                const now = new Date();

                // 30分以内の変更のみ復元
                if ((now.getTime() - lastModified.getTime()) <= AUTO_SAVE_EXPIRATION) {
                    if (parsedData.start_time) setStartTime(new Date(parsedData.start_time));
                    if (parsedData.end_time) setEndTime(new Date(parsedData.end_time));
                    if (parsedData.memo) setMemo(parsedData.memo);
                    if (parsedData.break_time) setBreakTime(parsedData.break_time);

                    toast({ description: "未保存の変更を復元しました" });
                    restoreState(lastModified);
                    return true;
                } else {
                    // 時間経過している場合はローカルストレージをクリア
                    localStorage.removeItem(`workLog_autosave_${projectId}`);
                }
            }
        } catch (error) {
            console.error("ローカルストレージからの復元に失敗しました:", error);
        }
        return false;
    }, [setStartTime, setEndTime, setMemo, setBreakTime, restoreState, toast]);

    // プロジェクト読み込み
    useEffect(() => {
        if (!state.isOpen || !state.projectId) {
            // ダイアログが閉じられた時のクリーンアップ
            prevProjectId.current = null;
            return;
        }

        // プロジェクトが未取得、もしくは異なるプロジェクトの場合のみ取得
        if (!project || project.id !== state.projectId) {
            const getProject = async () => {
                try {
                    const fetchedProject = await getProjectById(state.projectId!);
                    setProject(fetchedProject);
                    prevProjectId.current = state.projectId;
                    restoreFromLocalStorage(state.projectId!);
                } catch (error) {
                    console.error("プロジェクトの取得に失敗しました:", error);
                    toast({
                        variant: "destructive",
                        description: "プロジェクト情報の取得に失敗しました"
                    });
                }
            };

            getProject();
        }
    }, [state.isOpen, state.projectId, project, restoreFromLocalStorage, toast]);

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

    }, [state.isOpen, state.clickPosition]);

    // クリーンアップ処理
    const handleClose = useCallback(() => {
        setIsVisible(false);

        // アニメーション完了後にダイアログを閉じる
        setTimeout(() => {
            closeWorkLog();
            resetWorkLog();
            setProject(null);
            setDialogPosition(null);
            prevProjectId.current = null;

            // ローカルストレージのクリーンアップ
            if (project?.id) {
                localStorage.removeItem(`workLog_autosave_${project.id}`);
            }
        }, 200);
    }, [closeWorkLog, resetWorkLog, project?.id]);

    // 作業開始処理
    const handleStartWork = async () => {
        const now = new Date();
        setStartTime(now);
        setEndTime(null);

        if (!project?.id) return;

        try {
            const result = await createWorkLogAction({
                project_id: project.id,
                start_time: now.toISOString(),
                memo: state.memo,
                break_time: state.breakTime
            });

            if (!result.success) throw new Error(result.error);
            setWorkLogId(result.data.id);
        } catch (error) {
            console.error('稼働開始の記録に失敗しました:', error);
            setStartTime(null);
        }
    };

    // 作業終了処理
    const handleEndWork = async () => {
        if (state.endTime) return;
        const now = new Date();

        const validation = validateWorkLogTimes(state.startTime, now);
        if (!validation.isValid) {
            toast({
                variant: "destructive",
                description: validation.error
            });
            return;
        }

        if (!project?.id || !state.workLogId) return;

        try {
            const currentWorkTime = calculateWorkTime(state.startTime, state.endTime, state.breakTime);
            const result = await updateWorkLogAction(state.workLogId, {
                project_id: project.id,
                start_time: state.startTime!.toISOString(),
                end_time: now.toISOString(),
                memo: state.memo,
                break_time: state.breakTime,
                actual_work_minutes: currentWorkTime?.totalMinutes
            });

            if (!result.success) throw new Error(result.error);
            setEndTime(now);
        } catch (error) {
            console.error('稼働終了の記録に失敗しました:', error);
            toast({
                variant: "destructive",
                description: "稼働終了の記録に失敗しました"
            });
        }
    };

    const handleTimeEdit = async (type: 'start' | 'end', newDateTime: Date, isValid: boolean) => {
        if (!isValid || !project?.id || !state.workLogId) return;

        try {
            // 状態のみ更新（APIの呼び出しはuseAutoSaveに任せる）
            if (type === 'start') {
                setStartTime(newDateTime);
            } else {
                setEndTime(newDateTime);
            }

            toast({
                description: "日時を更新しました",
            });
        } catch (error) {
            console.error('日時の更新に失敗しました:', error);
            // エラー時は状態を元に戻す
            if (type === 'start') {
                setStartTime(state.startTime);
            } else {
                setEndTime(state.endTime);
            }
            toast({
                variant: "destructive",
                description: "日時の更新に失敗しました",
            });
        }
    };

    const handlePause = () => {
        if (!state.isPaused) {
            const now = new Date();
            setPauseStatus(true, now);
        } else {
            const pauseDuration = Math.floor(
                (new Date().getTime() - state.pauseStartTime!.getTime()) / 60000
            );
            setBreakTime(state.breakTime + pauseDuration);
            setPauseStatus(false, null);
        }
    };

    // 作業時間計算（メモ化）
    const workTime = useCallback(() => {
        return calculateWorkTime(state.startTime, state.endTime, state.breakTime);
    }, [state.startTime, state.endTime, state.breakTime]);

    // スタイル計算をメモ化（レンダリング毎に再計算されるのを防ぐ）
    const dialogStyle = useMemo(() => ({
        top: `${position.y}px`,
        left: `${position.x}px`,
        width: `${DIALOG_DIMENSIONS.width}px`,
        zIndex: 50,
        transition: isDragging ? 'none' :
            `transform ${DIALOG_DIMENSIONS.animationDuration}ms cubic-bezier(0.16, 1, 0.3, 1), opacity ${DIALOG_DIMENSIONS.animationDuration}ms cubic-bezier(0.16, 1, 0.3, 1)`,
        visibility: isVisible && dialogPosition ? 'visible' : 'hidden'
    }), [position.y, position.x, isDragging, isVisible, dialogPosition]);

    // メモ入力のハンドラをメモ化（イベントハンドラの再生成を防ぐ）
    const handleMemoChange = useCallback((e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setMemo(e.target.value);
    }, [setMemo]);

    // 日付表示のメモ化（毎回のformat処理を防ぐ）
    const formattedDate = useMemo(() =>
        format(new Date(), 'yyyy年M月d日（E）', { locale: ja }),
        []
    );

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
                ...dialogStyle,
                visibility: isVisible && dialogPosition ? 'visible' : 'hidden' as const
            }}
            onMouseDown={handleMouseDown}
        >
            <Header project={project} onClose={handleClose} />

            {project ? (
                <div className="space-y-4 p-4 animate-fade-in">
                    <div className="text-center text-sm text-gray-600 mb-2">
                        {formattedDate}の稼働記録
                    </div>

                    <ActionButtons
                        startTime={state.startTime}
                        endTime={state.endTime}
                        isPaused={state.isPaused}
                        onStart={handleStartWork}
                        onEnd={handleEndWork}
                        onPause={handlePause}
                        isToday={isToday(new Date())}
                    />

                    <TimeInfo
                        startTime={state.startTime}
                        endTime={state.endTime}
                        breakTime={state.breakTime}
                        workTime={workTime()}
                        onTimeEdit={handleTimeEdit}
                    />

                    <div className="space-y-2">
                        <Textarea
                            id="memo"
                            value={state.memo}
                            onChange={handleMemoChange}
                            placeholder="作業内容を入力してください"
                            className="h-24 resize-none text-primary"
                        />
                    </div>

                    {lastAutoSave && (
                        <div className="text-xs text-gray-400 text-center">
                            {isSaving ? "保存中..." :
                                isDirty ? '未保存の変更があります' :
                                    `最終保存: ${format(lastAutoSave, 'HH:mm')}`}
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
};

export default WorkLogDialog;
