import React from 'react';
import { Button } from '@/components/ui/button';
import { DialogFooter } from '@/components/ui/dialog';

interface ProjectDialogFooterProps {
    onSave: () => void;
    isDisabled: boolean;
}

export const ProjectDialogFooter = React.memo(({ onSave, isDisabled }: ProjectDialogFooterProps) => {
    return (
        <DialogFooter className="border-t border-dialog-hover pt-4">
            <Button
                onClick={onSave}
                disabled={isDisabled}
                className="hover:bg-text-primary/80"
            >
                開発プロジェクトを追加→
            </Button>
        </DialogFooter>
    );
});

ProjectDialogFooter.displayName = 'ProjectDialogFooter';
