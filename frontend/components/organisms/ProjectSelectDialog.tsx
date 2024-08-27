"use client";
import { useState, useEffect, useMemo } from 'react';
import MoleculesDialog from '@/components/molecules/dialog/MoleculesDialog';
import { Menu } from '@headlessui/react'
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
    base: 'py-2 px-3 cursor-pointer hover:bg-secondary',
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
    const [hoverIndex, setHoverIndex] = useState<number>();
    const [selectedPresetGroup, setSelectedPresetGroup] = useState<string>();

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

    function handleSetPreset(value: string) {
        onChange(value);
        onOpenChange(false);
        onClose();
    }

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
                        <button
                            key={key}
                            className={categoryButton({ selected: selectedPresetGroup === categoryPresets.company })}
                            onClick={() => setSelectedPresetGroup(categoryPresets.company)}
                        >
                            {categoryPresets.company}
                        </button>
                    ))}
                </div>
                <hr className="my-2 border-secondary" />
                {categoryGroupPreset.map((categoryPresets, presetsKey) => (
                    categoryPresets.company === selectedPresetGroup && (
                        <ul key={presetsKey} className="space-y-1">
                            {categoryPresets.items.map((item, itemsKey) => (
                                <li
                                    key={itemsKey}
                                    className={listItem({ selected: item.id === value })}
                                    onMouseEnter={() => setHoverIndex(itemsKey)}
                                    onMouseLeave={() => setHoverIndex(undefined)}
                                    onClick={() => handleSetPreset(item.id)}
                                >
                                    <div className="flex justify-between items-center">
                                        <Menu as="div" className="relative inline-block text-left">
                                            <Menu.Button className="inline-flex w-full justify-center items-center">
                                                {item.name}
                                            </Menu.Button>
                                            <Menu.Items className="absolute left-0 mt-2 w-56 origin-top-left divide-y divide-secondary rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                                                <div className="px-1 py-1">
                                                    <Menu.Item>
                                                        {({ active }) => (
                                                            <button
                                                                className={`${active ? 'bg-primary text-white' : 'text-text-primary'
                                                                    } group flex w-full items-center rounded-md px-2 py-2 text-sm`}
                                                            >
                                                                {item.name}
                                                            </button>
                                                        )}
                                                    </Menu.Item>
                                                </div>
                                            </Menu.Items>
                                        </Menu>
                                        {hoverIndex === itemsKey && (
                                            <span className="text-accent">選択</span>
                                        )}
                                    </div>
                                </li>
                            ))}
                        </ul>
                    )
                ))}
            </div>
        </MoleculesDialog>
    );
}