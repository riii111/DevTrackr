import React, { memo, useState, useCallback } from 'react';
import { Coffee, PencilIcon } from "lucide-react";
import { format } from 'date-fns';
import { z } from "zod";
import { TimeInput } from './TimeInput';

type TimeType = 'start' | 'end';

interface WorkTime {
    hours: number;
    minutes: number;
}

interface TimeInfoProps {
    startTime: Date | null;
    endTime: Date | null;
    breakTime: number;
    workTime: WorkTime | null;
    onTimeEdit: (type: TimeType, newDateTime: Date, isValid: boolean) => void;
}

const timeValidationSchema = z.object({
    startTime: z.date(),
    endTime: z.date().nullable(),
}).refine(data => {
    if (data.endTime === null) return true;
    return data.startTime < data.endTime;
}, {
    message: "終了時刻は開始時刻より後に設定してください",
}).refine(data => {
    // 未来の日時をチェック
    const now = new Date();
    if (data.startTime > now || (data.endTime && data.endTime > now)) {
        return false;
    }
    return true;
}, {
    message: "未来の日時は設定できません",
}).refine(data => {
    // 極端に過去の日時をチェック（例：30日以上前）
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    if (data.startTime < thirtyDaysAgo || (data.endTime && data.endTime < thirtyDaysAgo)) {
        return false;
    }
    return true;
}, {
    message: "30日以上前の日時は設定できません",
}).refine(data => {
    if (data.endTime === null) return true;
    const diffHours = (data.endTime.getTime() - data.startTime.getTime()) / (1000 * 60 * 60);
    return diffHours <= 24;
}, {
    message: "作業時間は24時間以内に収めてください",
});

export const TimeInfo = memo(({ startTime, endTime, breakTime, workTime, onTimeEdit }: TimeInfoProps) => {
    const [editingTime, setEditingTime] = useState<TimeType | null>(null);
    const [tempDateTime, setTempDateTime] = useState<Date | null>(null);
    const [validationError, setValidationError] = useState<string | null>(null);

    const handleEditClick = useCallback((type: TimeType) => {
        setEditingTime(type);
        setTempDateTime(type === 'start' ? startTime : endTime);
    }, [startTime, endTime]);

    // 日時変更時のバリデーション処理
    const handleTimeChange = useCallback((date: Date) => {
        if (isNaN(date.getTime())) {
            setValidationError("無効な日時です");
            return;
        }

        setTempDateTime(date);
        setValidationError(null);
    }, []);

    const handleBlur = () => {
        if (!editingTime || !tempDateTime) {
            setEditingTime(null);
            return;
        }

        try {
            // 入力値が不完全な場合は処理を中断
            if (!tempDateTime || isNaN(tempDateTime.getTime())) {
                setValidationError("日時を完全に入力してください");
                return;
            }

            const validationData = {
                startTime: editingTime === 'start' ? tempDateTime : (startTime || new Date()),
                endTime: editingTime === 'end' ? tempDateTime : endTime,
            };

            // バリデーションチェック
            timeValidationSchema.parse(validationData);
            setValidationError(null);

            // 親コンポーネントに有効な値として通知
            onTimeEdit(editingTime, tempDateTime, true);
            setEditingTime(null);
        } catch (error) {
            if (error instanceof z.ZodError) {
                const errorMessage = error.errors[0].message;
                setValidationError(errorMessage);
            }
        }
    };

    return (
        <div className="space-y-2 text-sm">
            <div className="flex items-center justify-between">
                <span className="text-gray-500">開始:</span>
                {editingTime === 'start' ? (
                    <TimeInput
                        value={tempDateTime}
                        onChange={handleTimeChange}
                        onBlur={handleBlur}
                        hasError={!!validationError}
                    />
                ) : (
                    <button
                        onClick={() => handleEditClick('start')}
                        className="text-right text-primary hover:text-primary focus:outline-none group flex items-center"
                    >
                        <span>{startTime ? format(startTime, 'M/d HH:mm') : '--:--'}</span>
                        <PencilIcon className="w-3 h-3 ml-1 text-gray-400 group-hover:text-primary" />
                    </button>
                )}
            </div>

            <div className="flex items-center justify-between">
                <span className="text-gray-500">終了:</span>
                {editingTime === 'end' ? (
                    <TimeInput
                        value={tempDateTime}
                        onChange={handleTimeChange}
                        onBlur={handleBlur}
                        hasError={!!validationError}
                    />
                ) : (
                    <button
                        onClick={() => handleEditClick('end')}
                        className="text-right text-primary hover:text-primary focus:outline-none group flex items-center"
                        disabled={!startTime}
                    >
                        <span>{endTime ? format(endTime, 'M/d HH:mm') : '--:--'}</span>
                        <PencilIcon className="w-3 h-3 ml-1 text-gray-400 group-hover:text-primary" />
                    </button>
                )}
            </div>

            {breakTime > 0 && (
                <div className="flex items-center gap-2 text-sm text-gray-600">
                    <Coffee className="h-4 w-4" />
                    <span>休憩時間: {breakTime}分</span>
                </div>
            )}

            {workTime && (
                <div className="text-sm font-medium text-primary">
                    実作業時間: {workTime.hours}時間 {workTime.minutes}分
                </div>
            )}

            {validationError && (
                <div className="text-xs text-red-500 mt-1">
                    {validationError}
                </div>
            )}
        </div>
    );
});

TimeInfo.displayName = 'TimeInfo';