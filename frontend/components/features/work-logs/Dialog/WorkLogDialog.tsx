'use client';

import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { X, Play, Square, GripHorizontal, PauseCircle, Edit2, Save, Coffee } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';
import { useWorkLog } from '@/lib/store/useWorkLogStore';
import { getProjectById } from "@/lib/api/projects";
import { ProjectResponse } from "@/types/project";
import { useAutoSave } from '@/lib/hooks/useAutoSave';
import { useDraggable } from '@/lib/hooks/useDraggable';

const styles = {
    slideIn: 'animate-slide-in',
    fadeIn: 'animate-fade-in',
    press: 'animate-press',
    bounce: 'animate-bounce-in',
} as const;

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

    const TimeDisplay = ({ time, label, canEdit }: { time: Date | null, label: string, canEdit: boolean }) => (
        <div className="flex items-center justify-between text-sm text-gray-600">
            <span>{label}:</span>
            <div className="flex items-center gap-2">
                {time ? (
                    <>
                        <span>{format(time, 'HH:mm', { locale: ja })}</span>
                        {canEdit && (
                            <Button
                                variant="ghost"
                                size="icon"
                                className="h-6 w-6"
                                onClick={() => handleTimeEdit(label.toLowerCase() as 'start' | 'end')}
                            >
                                <Edit2 className="h-3 w-3" />
                            </Button>
                        )}
                    </>
                ) : '未設定'}
            </div>
        </div>
    );

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
            <div className="space-y-4 pb-4 border-b animate-fade-in">
                <div className="flex items-center cursor-grab drag-handle select-none p-4">
                    <GripHorizontal className="h-5 w-5 text-gray-400 flex-shrink-0 animate-bounce-in" />

                    <div className="flex-1 mx-4">
                        <h2 className="text-lg font-bold text-primary text-center animate-fade-in">
                            稼働記録
                        </h2>
                        {project && (
                            <div className="mt-1 text-center animate-slide-in">
                                <Badge
                                    variant="secondary"
                                    className="max-w-full truncate px-3 py-1 text-sm bg-gray-100 text-gray-700 
                                             hover:bg-gray-200 transition-all duration-200 ease-in-out
                                             hover:scale-105 transform"
                                >
                                    {project.title}
                                </Badge>
                            </div>
                        )}
                    </div>

                    <Button
                        variant="ghost"
                        size="icon"
                        onClick={handleClose}
                        className="rounded-full hover:bg-gray-100 flex-shrink-0 h-8 w-8
                                 transition-all duration-200 ease-in-out
                                 hover:rotate-90 transform"
                    >
                        <X className="h-4 w-4 text-gray-500" />
                    </Button>
                </div>
            </div>

            <style jsx global>{`
                @keyframes slideIn {
                    from {
                        transform: translateY(10px);
                        opacity: 0;
                    }
                    to {
                        transform: translateY(0);
                        opacity: 1;
                    }
                }

                @keyframes slideOut {
                    from {
                        transform: translateY(0);
                        opacity: 1;
                    }
                    to {
                        transform: translateY(10px);
                        opacity: 0;
                    }
                }

                @keyframes fadeIn {
                    from { opacity: 0; }
                    to { opacity: 1; }
                }

                @keyframes bounceIn {
                    0% {
                        transform: scale(0.3);
                        opacity: 0;
                    }
                    50% {
                        transform: scale(1.05);
                        opacity: 0.8;
                    }
                    70% { transform: scale(0.9); }
                    100% {
                        transform: scale(1);
                        opacity: 1;
                    }
                }

                .animate-slide-in {
                    animation: slideIn 0.3s ease-out forwards;
                }

                .animate-slide-out {
                    animation: slideOut 0.2s ease-in forwards;
                }

                .animate-fade-in {
                    animation: fadeIn 0.3s ease-out forwards;
                }

                .animate-bounce-in {
                    animation: bounceIn 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55) forwards;
                }

                .button-hover {
                    transition: all 0.2s ease-in-out;
                }
                
                .button-hover:hover {
                    transform: translateY(-1px);
                    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
                }
            `}</style>

            {project ? (
                <form onSubmit={handleSubmit} className="space-y-4 p-4 animate-fade-in">
                    <Card className="bg-gray-50 p-4">
                        <div className="flex justify-center space-x-4">
                            <Button
                                type="button"
                                onClick={handleStartWork}
                                disabled={!!startTime}
                                className="w-full button-hover"
                                variant={startTime ? "secondary" : "default"}
                            >
                                <Play className="mr-2 h-4 w-4" />
                                開始
                            </Button>
                            <Button
                                type="button"
                                onClick={handleEndWork}
                                disabled={!startTime || !!endTime}
                                className="w-full button-hover"
                                variant={endTime ? "secondary" : "default"}
                            >
                                <Square className="mr-2 h-4 w-4" />
                                終了
                            </Button>
                            {startTime && !endTime && (
                                <TooltipProvider>
                                    <Tooltip>
                                        <TooltipTrigger asChild>
                                            <Button
                                                type="button"
                                                onClick={handlePause}
                                                variant="outline"
                                                className="button-hover"
                                            >
                                                {isPaused ? <Play className="h-4 w-4" /> : <PauseCircle className="h-4 w-4" />}
                                            </Button>
                                        </TooltipTrigger>
                                        <TooltipContent>
                                            {isPaused ? '作業を再開' : '一時停止'}
                                        </TooltipContent>
                                    </Tooltip>
                                </TooltipProvider>
                            )}
                        </div>
                    </Card>

                    <div className="space-y-3 bg-gray-50 p-3 rounded-lg">
                        <TimeDisplay time={startTime} label="開始" canEdit={true} />
                        <TimeDisplay time={endTime} label="終了" canEdit={!!endTime} />

                        {breakTime > 0 && (
                            <div className="flex items-center gap-2 text-sm text-gray-600">
                                <Coffee className="h-4 w-4" />
                                <span>休憩時間: {breakTime}分</span>
                            </div>
                        )}

                        {calculateWorkTime() && (
                            <div className="text-sm font-medium text-primary">
                                実作業時間: {calculateWorkTime()!.hours}時間 {calculateWorkTime()!.minutes}分
                            </div>
                        )}
                    </div>

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
