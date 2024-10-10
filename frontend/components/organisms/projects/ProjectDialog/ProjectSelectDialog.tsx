"use client";
import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { Button } from '@/components/ui/button'
import {
    Dialog,
    DialogContent,
    DialogFooter,
} from '@/components/ui/dialog'
import { useRouter } from "next/navigation";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { CompanyWithProjects } from "@/types/company";
import { ProjectDialogHeader } from "@/components/organisms/projects/ProjectDialog/content/ProjectDialogHeader";
import { ProjectSelector } from "@/components/organisms/projects/ProjectDialog/content/ProjectSelector";

interface Props {
    isOpen: boolean;
    companiesWithProjects: CompanyWithProjects[];
    onOpenChange: (isOpen: boolean) => void;
    onClose: () => void;
}

export const ProjectSelectDialog: React.FC<Props> = React.memo(({
    isOpen,
    companiesWithProjects = [],
    onOpenChange,
    onClose,
}) => {
    const [selectedPresetGroup, setSelectedPresetGroup] = useState<string>();
    const [selectedProjectId, setSelectedProjectId] = useState<string | undefined>(undefined);
    const [isProcessing, setIsProcessing] = useState(false);
    const router = useRouter();
    const drawerStore = useDrawerStore();

    const handleProjectSelect = useCallback((projectId: string) => {
        setSelectedProjectId(projectId);
    }, []);

    const handleSave = useCallback(async () => {
        if (!selectedProjectId) {
            return;
        }

        try {
            setIsProcessing(true);

            await drawerStore.handleOpen("main", { id: "event", type: "event" });

            const projectDetails = companiesWithProjects
                .flatMap(company => company.projects)
                .find(project => project.id === selectedProjectId);

            if (projectDetails) {
                router.push(`/dashboard/projects?projectId=${projectDetails.id}`);
            }

            onOpenChange(false);
        } catch (error) {
            console.error(error);
        } finally {
            setIsProcessing(false);
        }
    }, [selectedProjectId, companiesWithProjects, drawerStore, router, onOpenChange]);

    const categoryGroupPreset = useMemo(() => {
        return companiesWithProjects.map((company) => ({
            company: company.company_name,
            items: company.projects || []
        }));
    }, [companiesWithProjects]);

    // ダイアログが開かれた時に選択状態をクリア
    useEffect(() => {
        if (isOpen) {
            setSelectedProjectId(undefined);
        }
    }, [isOpen]);

    // カテゴリーグループのプリセットが変更されたときに、最初の会社を選択状態にする
    useEffect(() => {
        if (categoryGroupPreset.length > 0 && !selectedPresetGroup) {
            setSelectedPresetGroup(categoryGroupPreset[0].company);
        }
    }, [categoryGroupPreset, selectedPresetGroup]);

    const handleDialogChange = useCallback((open: boolean) => {
        onOpenChange(open);
        if (!open) {
            onClose();
        }
    }, [onOpenChange, onClose]);

    console.log("called ProjectSelectDialog");

    return (
        <Dialog open={isOpen} onOpenChange={handleDialogChange}>
            <DialogContent className="sm:max-w-[640px] bg-dialog-bg text-text-primary border-none shadow-lg">
                <ProjectDialogHeader />
                <ProjectSelector
                    categoryGroupPreset={categoryGroupPreset}
                    selectedPresetGroup={selectedPresetGroup}
                    onSelectPresetGroup={setSelectedPresetGroup}
                    onSelectProject={handleProjectSelect}
                    selectedProjectId={selectedProjectId}
                />
                <DialogFooter className="border-t border-dialog-hover pt-4">
                    <Button
                        onClick={handleSave}
                        disabled={!selectedProjectId || isProcessing}
                        className="hover:text-accent-dark hover:bg-dialog-selected"
                    >
                        開発プロジェクトを追加→
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
});

ProjectSelectDialog.displayName = "ProjectSelectDialog"