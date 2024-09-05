"use client";

import dynamic from 'next/dynamic';
import { useState } from "react";
import AtomsButtonWithIcon from "@/components/atoms/button/AtomsButtonWithIcon";
import { GoPlus } from "react-icons/go";
import { useRouter } from "next/navigation";
// import { useSnackbar } from "@/lib/store/useSnackbarStore";
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { ProjectDrawer } from "@/components/organisms/projects/ProjectDrawer/content/ProjectDrawer";

const ProjectSelectDialog = dynamic(() => import("@/components/organisms/projects/ProjectDialog/ProjectSelectDialog"), {
    ssr: false,
});

// const ProjectDrawer = dynamic(() => import("@/components/organisms/projects/ProjectDrawer/content/ProjectDrawer"), {
//     ssr: false,
// });

interface Project {
    id: string;
    name: string;
    // 他のプロジェクト関連のプロパティを追加
}

export default function TimeTrackingClientComponents() {
    const [isOpen, setIsOpen] = useState(false);
    const [isDrawerOpen, setIsDrawerOpen] = useState(false);
    // TODO: プロジェクト一覧を取得するAPIが完成後、useMemoを適用して実装する?
    // const project_list = useProjectList();
    const project_list = [
        {
            companyId: 1,
            companyName: "会社A",
            projects: [
                { id: "1", name: "案件1" },
                { id: "2", name: "案件2" },
            ]
        },
        {
            companyId: 2,
            companyName: "会社B",
            projects: [
                { id: "3", name: "案件3" },
                { id: "4", name: "案件4" },
                { id: "5", name: "案件5" },
            ]
        },
        {
            companyId: 3,
            companyName: "会社C",
            projects: [
                { id: "6", name: "案件6" },
                { id: "7", name: "案件7" },
                { id: "8", name: "案件8" },
                { id: "9", name: "案件9" },
                { id: "10", name: "案件10" },
                { id: "11", name: "案件11" },
            ]
        }
    ];


    return (
        <>
            <div className="flex justify-start mb-4">
                <AtomsButtonWithIcon
                    icon={GoPlus}
                    text="勤怠を追加"
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
                companies={project_list}
                setIsDrawerOpen={setIsDrawerOpen}
            />
            {isDrawerOpen && <ProjectDrawer
                isOpen={isDrawerOpen}
                onOpenChange={setIsDrawerOpen}
            />}
        </>
    );
}