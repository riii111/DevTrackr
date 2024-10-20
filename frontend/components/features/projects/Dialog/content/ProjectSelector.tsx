import React from 'react';
import { Button } from '@/components/ui/button'
import { tv } from 'tailwind-variants'
import { Project } from "@/types/project";

const categoryButton = tv({
    base: 'px-4 py-1 text-sm rounded-full border text-white/80 bg-text-primary/70',
    variants: {
        selected: {
            true: 'bg-text-primary text-white font-medium hover:bg-text-primary',
            false: 'hover:bg-text-primary/90 hover:text-white/80',
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

interface ProjectSelectorProps {
    categoryGroupPreset: { company: string; items: Project[] }[];
    selectedPresetGroup: string | undefined;
    selectedProjectId: string | undefined;
    onSelectPresetGroup: (company: string) => void;
    onSelectProject: (projectId: string) => void;
}

export const ProjectSelector: React.FC<ProjectSelectorProps> = React.memo(({
    categoryGroupPreset,
    selectedPresetGroup,
    selectedProjectId,
    onSelectPresetGroup,
    onSelectProject
}) => {
    return (
        <div className="p-4">
            <div className="flex space-x-2 pb-2">
                {categoryGroupPreset.map((categoryPresets, key) => (
                    <Button
                        key={key}
                        variant="outline"
                        className={categoryButton({ selected: selectedPresetGroup === categoryPresets.company })}
                        onClick={() => onSelectPresetGroup(categoryPresets.company)}
                    >
                        {categoryPresets.company}
                    </Button>
                ))}
            </div>
            <hr className="my-2 border-secondary-dark" />
            {categoryGroupPreset.find(categoryPreset => categoryPreset.company === selectedPresetGroup)?.items && (
                <ul className="space-y-1">
                    {categoryGroupPreset.find(categoryPreset => categoryPreset.company === selectedPresetGroup)!.items.map((project) => (
                        <li
                            key={project.id}
                            className={listItem({ selected: project.id === selectedProjectId })}
                            onClick={() => onSelectProject(project.id)}
                        >
                            <div className="flex justify-between items-center w-full">
                                <span className="text-primary">{project.title}</span>
                                {project.id === selectedProjectId && (
                                    <span className="text-accent-dark">選択</span>
                                )}
                            </div>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
});

ProjectSelector.displayName = "ProjectSelector";