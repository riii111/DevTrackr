'use client';

import React from 'react';
import { Button } from "@/components/ui/button";
import { IoTimer } from "react-icons/io5";
import { useWorkLog } from '@/lib/store/useWorkLogStore';

type WorkLogButtonProps = {
    projectId: string;
};

export const WorkLogButton: React.FC<WorkLogButtonProps> = React.memo(({ projectId }) => {
    const { dispatch } = useWorkLog();

    const handleClick = React.useCallback(() => {
        dispatch({ type: 'OPEN_WORK_LOG', projectId });
    }, [dispatch, projectId]);

    return (
        <>
            <Button onClick={handleClick}>
                <IoTimer />
                稼働記録
            </Button>
        </>
    );
});

WorkLogButton.displayName = 'WorkLogButton';
