import React from 'react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";

interface TruncatedTextProps {
    text: string;
    maxLength?: number;
}

// テキストを指定された最大長に切り詰める
const truncateText = (text: string, maxLength: number) => {
    if (text.length <= maxLength) return text;
    return text.slice(0, maxLength) + '...';
};

export const TruncatedText: React.FC<TruncatedTextProps> = ({ text, maxLength = 50 }) => {
    const truncatedText = truncateText(text, maxLength);

    return (
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger className="text-left">
                    {truncatedText}
                </TooltipTrigger>
                <TooltipContent>
                    <p className="max-w-md">{text}</p>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    );
};