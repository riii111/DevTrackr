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
    value: string | undefined;
    isOpen: boolean;
    companies: Company[];
    onChange: (value: string | undefined) => void;
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

// メモ化されたDialogHeader
const MemoizedDialogHeader = React.memo(() => (
    <DialogHeader>
        <DialogTitle>プロジェクト・開発案件の選択</DialogTitle>
        <DialogDescription>
            以下のリストからプロジェクトまたは開発案件を選択してください。
        </DialogDescription>
    </DialogHeader>
));
MemoizedDialogHeader.displayName = 'MemoizedDialogHeader';

// メモ化されたDialogFooter
const MemoizedDialogFooter = React.memo(({ onConfirm, isDisabled }: { onConfirm: () => void; isDisabled: boolean }) => (
    <DialogFooter>
        <Button onClick={onConfirm} disabled={isDisabled} className="text-text-primary hover:bg-gray-200 bg-white shadow-none">
            <span className='text-primary hover:text-accent'>プロジェクトを追加→</span>
        </Button>
    </DialogFooter>
));

MemoizedDialogFooter.displayName = 'MemoizedDialogFooter';

const CategoryButton = React.memo(({ company, isSelected, onClick }: { company: string; isSelected: boolean; onClick: () => void }) => (
    <Button
        variant="outline"
        className={categoryButton({ selected: isSelected })}
        onClick={onClick}
    >
        {company}
    </Button>
));

CategoryButton.displayName = 'CategoryButton';

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
            {categoryGroupPreset.map((categoryPresets, presetsKey) => (
                categoryPresets.company === selectedPresetGroup && (
                    <ul key={presetsKey} className="space-y-1">
                        {categoryPresets.items.map((item) => (
                            <li
                                key={item.id}
                                className={listItem({ selected: item.id === selectedProjectId })}
                                onClick={() => onSelectProject(item.id)}
                            >
                                <div className="flex justify-between items-center w-full">
                                    <span className="text-text-primary">{item.name}</span>
                                    {selectedProjectId === item.id && (
                                        <span className="text-accent">選択</span>
                                    )}
                                </div>
                            </li>
                        ))}
                    </ul>
                )
            ))}
        </div>
    );
};

export default function ProjectSelectDialog({
    isOpen,
    companies = [],
    onChange,
    onOpenChange,
    onClose
}: Props) {
    const [selectedPresetGroup, setSelectedPresetGroup] = useState<string>();
    const [selectedProjectId, setSelectedProjectId] = useState<string | undefined>(undefined);

    const handleProjectSelect = useCallback((projectId: string) => {
        setSelectedProjectId(projectId);
    }, []);

    const handleConfirm = useCallback(() => {
        if (selectedProjectId) {
            onChange(selectedProjectId);
            onOpenChange(false);
            onClose();
            setSelectedProjectId(undefined);
        }
    }, [selectedProjectId, onChange, onOpenChange, onClose]);

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

    useEffect(() => {
        if (categoryGroupPreset.length > 0) {
            setSelectedPresetGroup(categoryGroupPreset[0].company);
        }
    }, [categoryGroupPreset]);

    return (
        <Dialog open={isOpen} onOpenChange={(open) => {
            onOpenChange(open);
            if (!open) onClose();
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
                <MemoizedDialogFooter onConfirm={handleConfirm} isDisabled={!selectedProjectId} />
            </DialogContent>
        </Dialog>
    );
}