import { memo } from "react";
import { format } from "date-fns";

interface TimeInputProps {
    value: Date | null;
    onChange: (date: Date) => void;
    onBlur: () => void;
    hasError: boolean;
}

export const TimeInput = memo(({ value, onChange, onBlur, hasError }: TimeInputProps) => {
    const inputClassName = `text-right w-48 focus:outline-none focus:ring-1 
        ${hasError ? 'ring-red-500 text-red-500' : 'ring-primary text-primary'}
        [&::-webkit-calendar-picker-indicator]:opacity-100
        [&::-webkit-calendar-picker-indicator]:hover:cursor-pointer`;

    return (
        <input
            type="datetime-local"
            value={value ? format(value, "yyyy-MM-dd'T'HH:mm") : ''}
            onChange={(e) => {
                const date = new Date(e.target.value);
                if (!isNaN(date.getTime())) {
                    onChange(date);
                }
            }}
            onBlur={onBlur}
            className={inputClassName}
            autoFocus
        />
    );
});

TimeInput.displayName = 'TimeInput';