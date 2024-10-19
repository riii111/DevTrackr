"use client";
import React, { useState, useEffect, useMemo, useCallback } from 'react';
import {
    Dialog,
    DialogContent,
} from '@/components/ui/dialog'
import { useRouter } from "next/navigation";
import { CompanyWithProjects } from "@/types/company";
import { ProjectDialogHeader } from "@/components/organisms/projects/ProjectDialog/content/ProjectDialogHeader";
import { ProjectSelector } from "@/components/organisms/projects/ProjectDialog/content/ProjectSelector";
import { ProjectDialogFooter } from "@/components/organisms/projects/ProjectDialog/content/ProjectDialogFooter";

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

    const handleProjectSelect = useCallback((projectId: string) => {
        setSelectedProjectId(projectId);
    }, []);

    const handleSave = useCallback(async () => {
        if (!selectedProjectId) {
            return;
        }

        try {
            setIsProcessing(true);

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
    }, [selectedProjectId, companiesWithProjects, router, onOpenChange]);

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
                <ProjectDialogFooter
                    onSave={handleSave}
                    isDisabled={!selectedProjectId || isProcessing}
                />
            </DialogContent>
        </Dialog>
    );
});

ProjectSelectDialog.displayName = "ProjectSelectDialog"