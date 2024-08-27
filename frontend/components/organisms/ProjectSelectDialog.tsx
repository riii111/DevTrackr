"use client";
import { useState, useEffect, useMemo } from 'react';
import MoleculesDialog from '@/components/molecules/dialog/MoleculesDialog';
import { Button } from '@headlessui/react'
import { tv } from 'tailwind-variants'
// import { ChevronDownIcon } from '@heroicons/react/20/solid'
// import { useOrganizationStore } from '@/stores/organizationStore';

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

export default function ProjectSelectDialog({
    value,
    isOpen,
    companies = [],
    onChange,
    onOpenChange,
    onClose
}: Props) {
    const [selectedPresetGroup, setSelectedPresetGroup] = useState<string>();
    const [selectedProjectId, setSelectedProjectId] = useState<string | undefined>(undefined);

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

    function handleProjectSelect(projectId: string) {
        setSelectedProjectId(projectId);
    }

    function handleConfirm() {
        if (selectedProjectId) {
            onChange(selectedProjectId);
            onOpenChange(false);
            onClose();
            // ダイアログが閉じられた時に選択状態をクリア
            setSelectedProjectId(undefined);
        }
    }

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
        <MoleculesDialog
            isOpen={isOpen}
            onClose={() => {
                onOpenChange(false);
                onClose();
            }}
            title="プロジェクト・開発案件の選択"
            noGutters
            width={640}
        >
            <div className="p-4 bg-background">
                <div className="flex space-x-2 pb-2">
                    {categoryGroupPreset.map((categoryPresets, key) => (
                        <Button
                            key={key}
                            className={categoryButton({ selected: selectedPresetGroup === categoryPresets.company })}
                            onClick={() => setSelectedPresetGroup(categoryPresets.company)}
                        >
                            {categoryPresets.company}
                        </Button>
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
                                    onClick={() => handleProjectSelect(item.id)}
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
                <div className='flex w-full justify-end mt-4'>
                    <Button
                        className="text-text-primary hover:bg-gray-200"
                        onClick={handleConfirm}
                        disabled={!selectedProjectId}
                    >
                        <span className='text-primary hover:text-accent'>プロジェクトを追加→</span>
                    </Button>
                </div>
            </div>
        </MoleculesDialog>
    );
}