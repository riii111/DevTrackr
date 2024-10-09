"use client";

import dynamic from 'next/dynamic';
import { useState } from "react";
import AtomsButtonWithIcon from "@/components/atoms/button/AtomsButtonWithIcon";
import { GoPlus } from "react-icons/go";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { Company } from "@/types/company";

const ProjectSelectDialog = dynamic(
    () => import("@/components/organisms/projects/ProjectDialog/ProjectSelectDialog").then(mod => mod.ProjectSelectDialog),
    {
        ssr: false,
    }
);

const ProjectDrawer = dynamic(() => import("@/components/organisms/projects/ProjectDrawer/content/ProjectDrawer").then(mod => mod.ProjectDrawer), {
    ssr: false,
});

interface ProjectsClientComponentsProps {
    companies: Company[];
}

export default function ProjectsClientComponents({ companies }: ProjectsClientComponentsProps) {
    const [isOpen, setIsOpen] = useState(false);
    const { drawerState } = useDrawerStore();

    return (
        <>
            <div className="flex justify-start mb-4">
                <AtomsButtonWithIcon
                    icon={GoPlus}
                    text="プロジェクトを追加"
                    iconColor='bg-#[5883D3]'
                    textColor='bg-#[5883D3]'
                    rounded={6}
                    loading={false}
                    disabled={false}
                    onClick={() => setIsOpen(true)}
                />
            </div>
            <ProjectSelectDialog
                isOpen={isOpen}
                onOpenChange={setIsOpen}
                onClose={() => setIsOpen(false)}
                companies={companies}
            />
            {drawerState.main.isOpen && <ProjectDrawer />}
        </>
    );
}