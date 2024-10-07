"use client";
import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { Button } from '@/components/ui/button'
import { tv } from 'tailwind-variants'
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
    DialogFooter,
} from '@/components/ui/dialog'
import { useRouter } from "next/navigation";
import { useDrawerStore } from "@/lib/store/useDrawerStore";

interface Project {
    id: string;
    name: string;
}

interface Company {
    companyId: number;
    companyName: string;
    projects: Project[];
}

interface Props {
    isOpen: boolean;
    companies: Company[];
    onOpenChange: (isOpen: boolean) => void;
    onClose: () => void;
}

const categoryButton = tv({
    base: 'px-4 py-1 text-sm rounded-full border',
    variants: {
        selected: {
            true: 'text-white font-medium hover:bg-text-primary',
            false: 'text-text-secondary hover:bg-text-primary/70 hover:text-white/80',
        },
    },
});

const listItem = tv({
    base: 'py-2 px-3 cursor-pointer text-text-primary rounded transition-colors duration-150',
    variants: {
        selected: {
            true: 'bg-dialog-selected',
            false: 'hover:bg-dialog-hover',
        },
    },
});

const MemoizedDialogHeader = React.memo(() => (
    <DialogHeader className="bg-dialog-header p-4 rounded-t-lg">
        <DialogTitle className="text-primary font-semibold">開発プロジェクトの選択</DialogTitle>
        <DialogDescription className="text-text-secondary mt-1">
            以下のリストから開発プロジェクトを選択してください。
        </DialogDescription>
    </DialogHeader>
));
MemoizedDialogHeader.displayName = 'MemoizedDialogHeader';

const MemoizedDialogFooter = React.memo(({ onConfirm, isDisabled }: { onConfirm: () => void; isDisabled: boolean }) => (
    <DialogFooter className="border-t border-dialog-hover pt-4">
        <Button
            onClick={onConfirm}
            disabled={isDisabled}
            className="hover:text-accent-dark hover:bg-dialog-selected"
        >
            開発プロジェクトを追加→
        </Button>
    </DialogFooter>
));
MemoizedDialogFooter.displayName = 'MemoizedDialogFooter';

const CategoryButton = ({ company, isSelected, onClick }: { company: string; isSelected: boolean; onClick: () => void }) => (
    <Button
        variant="outline"
        className={categoryButton({ selected: isSelected })}
        onClick={onClick}
    >
        {company}
    </Button>
);

const ProjectList = ({ projects, selectedProjectId, onSelectProject }: { projects: Project[]; selectedProjectId: string | undefined; onSelectProject: (projectId: string) => void }) => (
    <ul className="space-y-1">
        {projects.map((project) => (
            <li
                key={project.id}
                className={listItem({ selected: project.id === selectedProjectId })}
                onClick={() => onSelectProject(project.id)}
            >
                <div className="flex justify-between items-center w-full">
                    <span className="text-primary">{project.name}</span>
                    {project.id === selectedProjectId && (
                        <span className="text-accent-dark">選択</span>
                    )}
                </div>
            </li>
        ))}
    </ul>
);

const ProjectSelector = React.memo(({
    categoryGroupPreset,
    selectedPresetGroup,
    selectedProjectId,
    onSelectPresetGroup,
    onSelectProject
}: {
    categoryGroupPreset: { company: string; items: Project[] }[];
    selectedPresetGroup: string | undefined;
    selectedProjectId: string | undefined;
    onSelectPresetGroup: (company: string) => void;
    onSelectProject: (projectId: string) => void;
}) => {
    return (
        <div className="p-4">
            <div className="flex space-x-2 pb-2">
                {categoryGroupPreset.map((categoryPresets, key) => (
                    <CategoryButton
                        key={key}
                        company={categoryPresets.company}
                        isSelected={selectedPresetGroup === categoryPresets.company}
                        onClick={() => onSelectPresetGroup(categoryPresets.company)}
                    />
                ))}
            </div>
            <hr className="my-2 border-secondary-dark" />
            {categoryGroupPreset.find(categoryPreset => categoryPreset.company === selectedPresetGroup)?.items && (
                <ProjectList
                    projects={categoryGroupPreset.find(categoryPreset => categoryPreset.company === selectedPresetGroup)!.items}
                    selectedProjectId={selectedProjectId}
                    onSelectProject={onSelectProject}
                />
            )}
        </div>
    );
});
ProjectSelector.displayName = "ProjectSelector"

export const ProjectSelectDialog: React.FC<Props> = React.memo(({
    isOpen,
    companies = [],
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

            const projectDetails = companies
                .flatMap(company => company.projects)
                .find(project => project.id === selectedProjectId);

            if (projectDetails) {
                router.push(`/dashboard/time-tracking?projectId=${projectDetails.id}`);
            }

            onOpenChange(false);
        } catch (error) {
            console.error(error);
        } finally {
            setIsProcessing(false);
        }
    }, [selectedProjectId, companies, drawerStore, router, onOpenChange]);

    // TODO: ダミーの処理. 会社ごとに案件を表示させるように修正する.
    const categoryGroupPreset = useMemo(() => {
        const presetCompanies = ["会社A", "会社B", "会社C"];

        const allCompanyNames = Array.from(new Set([
            ...presetCompanies,
            ...companies.map(company => company.companyName)
        ]));

        return allCompanyNames.map((companyName) => ({
            company: companyName,
            items: companies.find(c => c.companyName === companyName)?.projects || []
        }));
    }, [companies]);

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
                <MemoizedDialogHeader />
                <ProjectSelector
                    categoryGroupPreset={categoryGroupPreset}
                    selectedPresetGroup={selectedPresetGroup}
                    onSelectPresetGroup={setSelectedPresetGroup}
                    onSelectProject={handleProjectSelect}
                    selectedProjectId={selectedProjectId}
                />
                <MemoizedDialogFooter onConfirm={handleSave} isDisabled={!selectedProjectId || isProcessing} />
            </DialogContent>
        </Dialog>
    );
});

ProjectSelectDialog.displayName = "ProjectSelectDialog"