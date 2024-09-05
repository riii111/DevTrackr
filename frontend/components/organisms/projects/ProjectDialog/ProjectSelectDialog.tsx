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
    base: 'px-4 py-1 text-sm rounded-full',
    variants: {
        selected: {
            true: 'bg-primary text-white',
            false: 'bg-secondary text-text-secondary',
        },
    },
});

const listItem = tv({
    base: 'py-2 px-3 cursor-pointer hover:bg-secondary text-text-primary',
    variants: {
        selected: {
            true: 'bg-secondary',
            false: '',
        },
    },
});

const MemoizedDialogHeader = React.memo(() => (
    <DialogHeader>
        <DialogTitle>開発プロジェクトの選択</DialogTitle>
        <DialogDescription>
            以下のリストから開発プロジェクトを選択してください。
        </DialogDescription>
    </DialogHeader>
));
MemoizedDialogHeader.displayName = 'MemoizedDialogHeader';

const MemoizedDialogFooter = React.memo(({ onConfirm, isDisabled }: { onConfirm: () => void; isDisabled: boolean }) => (
    <DialogFooter>
        <Button onClick={onConfirm} disabled={isDisabled} className="text-text-primary hover:bg-gray-200 bg-white shadow-none">
            <span className='text-primary hover:text-accent'>開発プロジェクトを追加→</span>
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
                    <span className="text-text-primary">{project.name}</span>
                    {project.id === selectedProjectId && (
                        <span className="text-accent">選択</span>
                    )}
                </div>
            </li>
        ))}
    </ul>
);

// プロジェクト選択部分を別コンポーネントに分離
const ProjectSelector = ({
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
        <div className="p-4 bg-background">
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
};

export default function ProjectSelectDialog({
    isOpen,
    companies = [],
    onOpenChange,
    onClose,
}: Props) {
    const [selectedPresetGroup, setSelectedPresetGroup] = useState<string>();
    const [selectedProjectId, setSelectedProjectId] = useState<string | undefined>(undefined);
    const [isProcessing, setIsProcessing] = useState(false);
    const router = useRouter();
    const drawerStore = useDrawerStore();

    const handleProjectSelect = useCallback((projectId: string) => {
        setSelectedProjectId(projectId);
    }, []);

    const handleConfirm = useCallback(() => {
        if (selectedProjectId) {
            handleSave();
        }
    }, [selectedProjectId]);

    const handleSave = async () => {
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
    };

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
        if (categoryGroupPreset.length > 0) {
            setSelectedPresetGroup(categoryGroupPreset[0].company);
        }
    }, [categoryGroupPreset]);

    return (
        <Dialog open={isOpen} onOpenChange={(open) => {
            onOpenChange(open);
            if (!open) {
                onClose();
            }
        }}>
            <DialogContent className="sm:max-w-[640px]">
                <MemoizedDialogHeader />
                <ProjectSelector
                    categoryGroupPreset={categoryGroupPreset}
                    selectedPresetGroup={selectedPresetGroup}
                    onSelectPresetGroup={setSelectedPresetGroup}
                    onSelectProject={handleProjectSelect}
                    selectedProjectId={selectedProjectId}
                />
                <MemoizedDialogFooter onConfirm={handleConfirm} isDisabled={!selectedProjectId || isProcessing} />
            </DialogContent>
        </Dialog>
    );
}