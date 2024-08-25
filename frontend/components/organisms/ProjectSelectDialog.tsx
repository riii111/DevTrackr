"use client";
import { useState, useEffect, useMemo } from 'react';
import MoleculesDialog from '@/components/molecules/dialog/MoleculesDialog';
import { Menu, MenuItem, MenuItems } from '@headlessui/react'
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
            <div className="p-4">
                <div className="flex space-x-2 pb-2">
                    {categoryGroupPreset.map((categoryPresets, key) => (
                        <button
                            key={key}
                            className={`px-4 py-1 text-sm rounded-full ${selectedPresetGroup === categoryPresets.company
                                ? 'bg-blue-500 text-white'
                                : 'bg-gray-200 text-gray-700'
                                }`}
                            onClick={() => setSelectedPresetGroup(categoryPresets.company)}
                        >
                            {categoryPresets.company}
                        </button>
                    ))}
                </div>
                <hr className="my-2" />
                {categoryGroupPreset.map((categoryPresets, presetsKey) => (
                    categoryPresets.company === selectedPresetGroup && (
                        <ul key={presetsKey} className="space-y-1">
                            {categoryPresets.items.map((item, itemsKey) => (
                                <li
                                    key={itemsKey}
                                    className={`py-2 px-3 cursor-pointer ${item.id === value ? 'bg-gray-100' : ''
                                        } hover:bg-gray-50`}
                                    onMouseEnter={() => setHoverIndex(itemsKey)}
                                    onMouseLeave={() => setHoverIndex(undefined)}
                                    onClick={() => handleSetPreset(item.id)}
                                >
                                    <div className="flex justify-between items-center">
                                        <Menu as="div" className="relative inline-block text-left">
                                            <Menu.Button className="inline-flex w-full justify-center items-center">
                                                {item.name}
                                            </Menu.Button>
                                            <Menu.Items className="absolute left-0 mt-2 w-56 origin-top-left divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                                                <div className="px-1 py-1">
                                                    <Menu.Item>
                                                        {({ active }) => (
                                                            <button
                                                                className={`${active ? 'bg-blue-500 text-white' : 'text-gray-900'
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
                                            <span className="text-blue-500">選択</span>
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