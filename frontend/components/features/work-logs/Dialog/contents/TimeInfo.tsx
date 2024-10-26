import React, { memo } from 'react';
import { Button } from "@/components/ui/button";
import { Coffee, Edit2 } from "lucide-react";
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';

interface TimeInfoProps {
    startTime: Date | null;
    endTime: Date | null;
    breakTime: number;
    workTime: { hours: number; minutes: number } | null;
    onTimeEdit: (type: 'start' | 'end') => void;
}

export const TimeInfo: React.FC<TimeInfoProps> = memo(({
    startTime,
    endTime,
    breakTime,
    workTime,
    onTimeEdit
}) => {
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
                                onClick={() => onTimeEdit(label.toLowerCase() as 'start' | 'end')}
                            >
                                <Edit2 className="h-3 w-3" />
                            </Button>
                        )}
                    </>
                ) : '未設定'}
            </div>
        </div>
    );

    return (
        <div className="space-y-3 bg-gray-50 p-3 rounded-lg">
            <TimeDisplay time={startTime} label="開始" canEdit={true} />
            <TimeDisplay time={endTime} label="終了" canEdit={!!endTime} />

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
        </div>
    );
});

TimeInfo.displayName = 'TimeInfo';