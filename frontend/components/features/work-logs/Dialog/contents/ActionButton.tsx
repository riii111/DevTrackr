import React, { memo } from 'react';
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { Play, Square, PauseCircle } from "lucide-react";

interface ActionButtonsProps {
    startTime: Date | null;
    endTime: Date | null;
    isPaused: boolean;
    onStart: () => void;
    onEnd: () => void;
    onPause: () => void;
}

export const ActionButtons: React.FC<ActionButtonsProps> = memo(({
    startTime,
    endTime,
    isPaused,
    onStart,
    onEnd,
    onPause
}) => {
    return (
        <Card className="bg-gray-50 p-4">
            <div className="flex justify-center space-x-4">
                <Button
                    type="button"
                    onClick={onStart}
                    disabled={!!startTime}
                    className="w-full button-hover"
                    variant={startTime ? "secondary" : "default"}
                >
                    <Play className="mr-2 h-4 w-4" />
                    開始
                </Button>
                <Button
                    type="button"
                    onClick={onEnd}
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
                                    onClick={onPause}
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
    );
});

ActionButtons.displayName = 'ActionButtons';