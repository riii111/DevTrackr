import React, { memo, useState } from 'react';
import { Coffee, PencilIcon } from "lucide-react";
import { format } from 'date-fns';
import { z } from "zod";

interface TimeInfoProps {
    startTime: Date | null;
    endTime: Date | null;
    breakTime: number;
    workTime: { hours: number; minutes: number } | null;
    onTimeEdit: (type: 'start' | 'end', newDateTime: Date, isValid: boolean) => void;
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

export const TimeInfo = memo(
    ({ startTime, endTime, breakTime, workTime, onTimeEdit }: TimeInfoProps) => {
        const [editingTime, setEditingTime] = useState<'start' | 'end' | null>(null);
        const [tempDateTime, setTempDateTime] = useState<Date | null>(null);
        const [validationError, setValidationError] = useState<string | null>(null);

        const handleEditClick = (type: 'start' | 'end') => {
            setEditingTime(type);
            setTempDateTime(type === 'start' ? startTime : endTime);
        };

        const handleTimeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
            if (!e.target.value || e.target.value.length < 16) { // "yyyy-MM-ddTHH:mm"の長さをチェック
                setValidationError("日時を完全に入力してください");
                return;
            }

            const newDateTime = new Date(e.target.value);
            // 無効な日付の場合も早期リターン
            if (isNaN(newDateTime.getTime())) {
                setValidationError("無効な日時です");
                return;
            }

            setTempDateTime(newDateTime);
            setValidationError(null);
        };

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

        // 編集フィールドのスタイル
        const inputClassName = `text-right w-48 focus:outline-none focus:ring-1 
            ${validationError ? 'ring-red-500 text-red-500' : 'ring-primary text-primary'}
            [&::-webkit-calendar-picker-indicator]:opacity-100  // カレンダーアイコンを常に表示
            [&::-webkit-calendar-picker-indicator]:hover:cursor-pointer`;

        return (
            <div className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                    <span className="text-gray-500">開始:</span>
                    {editingTime === 'start' ? (
                        <input
                            type="datetime-local"
                            value={tempDateTime ? format(tempDateTime, "yyyy-MM-dd'T'HH:mm") : ''}
                            onChange={handleTimeChange}
                            onBlur={handleBlur}
                            className={inputClassName}
                            autoFocus
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
                        <input
                            type="datetime-local"
                            value={tempDateTime ? format(tempDateTime, "yyyy-MM-dd'T'HH:mm") : ''}
                            onChange={handleTimeChange}
                            onBlur={handleBlur}
                            className={inputClassName}
                            autoFocus
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