import React, { memo, useState } from 'react';
import { Coffee, PencilIcon } from "lucide-react";
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';

interface TimeInfoProps {
    startTime: Date | null;
    endTime: Date | null;
    breakTime: number;
    workTime: { hours: number; minutes: number } | null;
    onTimeEdit: (type: 'start' | 'end', newDateTime: Date) => void;
}

export const TimeInfo = memo(
    ({ startTime, endTime, breakTime, workTime, onTimeEdit }: TimeInfoProps) => {
        const [editingTime, setEditingTime] = useState<'start' | 'end' | null>(null);
        const [tempDateTime, setTempDateTime] = useState<Date | null>(null);

        const handleEditClick = (type: 'start' | 'end') => {
            setEditingTime(type);
            setTempDateTime(type === 'start' ? startTime : endTime);
        };

        const handleTimeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
            if (!tempDateTime) return;
            const newDateTime = new Date(e.target.value);
            setTempDateTime(newDateTime);
        };

        const handleBlur = () => {
            if (editingTime && tempDateTime) {
                onTimeEdit(editingTime, tempDateTime);
                setEditingTime(null);
            }
        };

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
                            className="text-right text-primary w-40 focus:outline-none focus:ring-1 focus:ring-primary"
                            autoFocus
                        />
                    ) : (
                        <button
                            onClick={() => handleEditClick('start')}
                            className="text-right text-primary hover:text-primary focus:outline-none group"
                        >
                            {startTime ? format(startTime, 'M/d HH:mm') : '--:--'}
                            <PencilIcon className="w-3 h-3 ml-1 inline opacity-0 group-hover:opacity-100" />
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
                            className="text-right text-primary w-40 focus:outline-none focus:ring-1 focus:ring-primary"
                            autoFocus
                        />
                    ) : (
                        <button
                            onClick={() => handleEditClick('end')}
                            className="text-right text-primary hover:text-primary focus:outline-none group"
                            disabled={!startTime}
                        >
                            {endTime ? format(endTime, 'M/d HH:mm') : '--:--'}
                            <PencilIcon className="w-3 h-3 ml-1 inline opacity-0 group-hover:opacity-100" />
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
            </div>
        );
    });

TimeInfo.displayName = 'TimeInfo';