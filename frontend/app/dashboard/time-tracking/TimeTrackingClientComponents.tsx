"use client";

import dynamic from 'next/dynamic';
import { useState } from "react";
import AtomsButtonWithIcon from "@/components/atoms/button/AtomsButtonWithIcon";
import { GoPlus } from "react-icons/go";
import { useRouter } from "next/navigation";
// import { useToast } from "@/components/ui/use-toast";

const ProjectSelectDialog = dynamic(() => import("@/components/organisms/ProjectSelectDialog"), {
    ssr: false,
});

export default function TimeTrackingClientComponents() {
    const [isOpen, setIsOpen] = useState(false);
    const [selectedProject, setSelectedProject] = useState<string>();
    const [isProcessing, setIsProcessing] = useState(false);
    const router = useRouter();

    // TODO: プロジェクト一覧を取得するAPIが完成後、useMemoを適用して実装する?
    // const project_list = useProjectList();
    const project_list = [
        {
            companyId: 1,
            companyName: "会社A",
            projects: [
                { id: "proj_1", name: "案件1" },
                { id: "proj_2", name: "案件2" },
            ]
        },
        {
            companyId: 2,
            companyName: "会社B",
            projects: [
                { id: "proj_3", name: "案件3" },
                { id: "proj_4", name: "案件4" },
                { id: "proj_5", name: "案件5" },
            ]
        },
        {
            companyId: 3,
            companyName: "会社C",
            projects: [
                { id: "proj_6", name: "案件6" },
                { id: "proj_7", name: "案件7" },
                { id: "proj_8", name: "案件8" },
                { id: "proj_9", name: "案件9" },
                { id: "proj_10", name: "案件10" },
                { id: "proj_11", name: "案件11" },
            ]
        }
    ];

    const handleSave = async () => {
        if (!selectedProject) {
            return;
        }

        try {
            setIsProcessing(true);

            // APIを呼び出して勤怠データを作成
            // const res = await projectStore.createTimeTracking({
            // projectId: selectedProject,
            // 他の必要なデータをここに追加
            // });

            // 成功通知
            // toast({
            //     title: "勤怠を追加しました",
            //     description: `プロジェクト: ${selectedProject}`,
            // });

            // ダイアログを閉じる
            setIsOpen(false);

            // 次の画面に遷移（プロジェクトの詳細画面）
            // router.push(`/dashboard/time-tracking/${res.id}`);

        } catch (error) {
            console.error(error);
            // toast({
            //     title: "エラー",
            //     description: "勤怠の作成に失敗しました",
            //     variant: "destructive",
            // });
        } finally {
            setIsProcessing(false);
        }
    };

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
                value={selectedProject}
                companies={project_list}
                onChange={setSelectedProject}
            />
        </>
    );
}