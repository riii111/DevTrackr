'use client';

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Save } from "lucide-react";
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

interface TimeEditMode {
    type: 'start' | 'end' | 'break';
    time: Date;
}

export function WorkLogDialog() {
    const { state, dispatch } = useWorkLog();
    const [project, setProject] = useState<ProjectResponse | null>(null);
    const [startTime, setStartTime] = useState<Date | null>(null);
    const [endTime, setEndTime] = useState<Date | null>(null);
    const [memo, setMemo] = useState("");
    const dialogRef = useRef<HTMLDivElement>(null);
    const prevProjectId = useRef<string | null>(null);

    // アニメーション状態の追加
    const [isVisible, setIsVisible] = useState(false);
    const [isTimeEditing, setIsTimeEditing] = useState<TimeEditMode | null>(null);
    const [breakTime, setBreakTime] = useState(0);
    const [isPaused, setIsPaused] = useState(false);
    const [pauseStartTime, setPauseStartTime] = useState<Date | null>(null);
    // use client宣言してるのになぜか"window is not defined"が出るので、初期値設定しておきクライアントサイドで操作する
    const [initialPosition, setInitialPosition] = useState({ x: 100, y: 100 });

    const { position, isDragging, handleMouseDown } = useDraggable(
        initialPosition,
        dialogRef
    );

    const { save, lastAutoSave } = useAutoSave({
        project_id: project?.id,
        start_time: startTime?.toISOString(),
        end_time: endTime?.toISOString(),
        memo,
    });

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

    useEffect(() => {
        if (state.isOpen) {
            setIsVisible(true);
        }
    }, [state.isOpen]);

    useEffect(() => {
        // クライアントサイドでのみ実行
        const updatePosition = () => {
            setInitialPosition({ x: window.innerWidth - 480, y: 100 });
        };

        updatePosition();
        window.addEventListener('resize', updatePosition);

        return () => window.removeEventListener('resize', updatePosition);
    }, []);

    const handleClose = useCallback(() => {
        setIsVisible(false);
        // アニメーション完了後にダイアログを閉じる
        setTimeout(() => {
            dispatch({ type: 'CLOSE_WORK_LOG' });
            setStartTime(null);
            setEndTime(null);
            setMemo("");
            setProject(null);
            prevProjectId.current = null;
        }, 200);
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

    const handleTimeEdit = (type: 'start' | 'end' | 'break') => {
        setIsTimeEditing({
            type,
            time: type === 'start' ? startTime! : endTime!
        });
    };

    const handleTimeUpdate = (newTime: Date) => {
        if (!isTimeEditing) return;

        switch (isTimeEditing.type) {
            case 'start':
                setStartTime(newTime);
                break;
            case 'end':
                setEndTime(newTime);
                break;
            case 'break':
                const diffInMinutes = Math.floor((newTime.getTime() - pauseStartTime!.getTime()) / 60000);
                setBreakTime(prev => prev + diffInMinutes);
                break;
        }
        setIsTimeEditing(null);
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
            setCurrentDate(new Date());
        }
    }, [state.isOpen]);

    if (!state.isOpen) return null;

    return (
        <div
            ref={dialogRef}
            className={`
                fixed bg-white shadow-xl rounded-lg overflow-hidden border border-gray-200
                ${isVisible ? 'animate-slide-in opacity-100' : 'animate-slide-out opacity-0'}
            `}
            style={{
                top: `${position.y}px`,
                left: `${position.x}px`,
                width: '384px',
                zIndex: 50,
                transition: isDragging ? 'none' : 'all 0.2s ease-in-out',
            }}
            onMouseDown={handleMouseDown}
        >
            <Header project={project} onClose={handleClose} />

            {project ? (
                <form onSubmit={handleSubmit} className="space-y-4 p-4 animate-fade-in">
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

                    {isTimeEditing && (
                        <div className="space-y-2">
                            <label className="text-sm font-medium">
                                {isTimeEditing.type === 'start' ? '開始' : '終了'}時刻を編集
                            </label>
                            <Input
                                type="time"
                                value={format(isTimeEditing.time, 'HH:mm')}
                                onChange={(e) => {
                                    const [hours, minutes] = e.target.value.split(':').map(Number);
                                    const newTime = new Date(isTimeEditing.time);
                                    newTime.setHours(hours, minutes);
                                    handleTimeUpdate(newTime);
                                }}
                                className="text-center text-primary"
                            />
                        </div>
                    )}

                    <div className="space-y-2">
                        <label htmlFor="memo" className="text-sm font-medium block">
                            メモ
                        </label>
                        <Textarea
                            id="memo"
                            value={memo}
                            onChange={(e) => setMemo(e.target.value)}
                            placeholder="作業内容を入力してください"
                            className="h-24 resize-none text-primary"
                        />
                    </div>

                    <Button
                        type="submit"
                        disabled={!startTime || !endTime}
                        className="w-full button-hover"
                    >
                        <Save className="mr-2 h-4 w-4" />
                        記録を保存
                    </Button>

                    {lastAutoSave && (
                        <div className="text-xs text-gray-400 text-center">
                            最終自動保存: {format(lastAutoSave, 'HH:mm')}
                        </div>
                    )}
                </form>
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
