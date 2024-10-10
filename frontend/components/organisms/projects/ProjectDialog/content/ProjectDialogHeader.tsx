import React from 'react';
import {
    DialogHeader,
    DialogTitle,
    DialogDescription,
} from '@/components/ui/dialog'

export const ProjectDialogHeader = React.memo(() => (
    <DialogHeader className="bg-dialog-header p-4 rounded-t-lg">
        <DialogTitle className="text-primary font-semibold">開発プロジェクトの選択</DialogTitle>
        <DialogDescription className="text-text-secondary mt-1">
            以下のリストから開発プロジェクトを選択してください。
        </DialogDescription>
    </DialogHeader>
));

ProjectDialogHeader.displayName = 'ProjectDialogHeader';