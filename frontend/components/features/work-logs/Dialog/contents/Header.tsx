import React, { memo } from 'react';
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { X, GripHorizontal } from "lucide-react";
import { ProjectResponse } from "@/types/project";
import { format } from 'date-fns';
import { ja } from 'date-fns/locale';

interface HeaderProps {
    project: ProjectResponse | null;
    onClose: () => void;
}

export const Header: React.FC<HeaderProps> = memo(({ project, onClose }) => {
    return (
        <div className="space-y-4 border-b animate-fade-in">
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
                    onClick={onClose}
                    className="rounded-full hover:bg-gray-100 flex-shrink-0 h-8 w-8
                             transition-all duration-200 ease-in-out
                             hover:rotate-90 transform"
                >
                    <X className="h-4 w-4 text-gray-500" />
                </Button>
            </div>
        </div>
    );
});

Header.displayName = 'Header';
