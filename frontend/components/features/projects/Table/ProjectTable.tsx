import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { MdChevronRight } from 'react-icons/md';
import Link from 'next/link';
import { Project, ProjectStatus } from "@/types/project";
import { SkillIcon } from "@/components/core/SkillIcon";
import { TruncatedText } from "@/components/core/TruncatedText";
import { StatusBadge } from "@/components/core/StatusBadge";
import { WorkLogButton } from "@/components/features/work-logs/Button/WorkLogButton";

interface ProjectTableProps {
    projects: Project[];
}

export const ProjectTable: React.FC<ProjectTableProps> = ({ projects }) => {
    if (!projects || projects.length === 0) {
        return <p>プロジェクトがありません。</p>;
    }

    return (
        <div className="overflow-x-auto">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead>タイトル</TableHead>
                        <TableHead className="w-1/3">説明</TableHead>
                        <TableHead>技術スタック</TableHead>
                        <TableHead>時給</TableHead>
                        <TableHead>ステータス</TableHead>
                        <TableHead>総作業時間</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {projects.map((project) => {
                        const totalMinutes = Math.floor(project.total_working_time / 60);
                        const hours = Math.floor(totalMinutes / 60);
                        const minutes = totalMinutes % 60;

                        return (
                            <TableRow
                                key={project.id.toString()}
                                className="group hover:bg-gray-50 transition-colors"
                            >
                                <TableCell className="relative p-0">
                                    <Link
                                        href={`/dashboard/projects?projectId=${project.id}`}
                                        className="flex items-center w-full h-full p-4 text-gray-900 hover:text-blue-600 transition-colors font-semibold"
                                    >
                                        <span>{project.title}</span>
                                        <MdChevronRight className="ml-2 h-6 w-6 text-gray-500 group-hover:text-blue-600" />
                                    </Link>
                                </TableCell>
                                <TableCell>
                                    <TruncatedText text={project.description || "-"} />
                                </TableCell>
                                <TableCell>
                                    <div className="flex flex-wrap gap-2">
                                        {project.skill_labels?.map((skill) => (
                                            <SkillIcon key={skill} skill={skill} />
                                        ))}
                                    </div>
                                </TableCell>
                                <TableCell>{project.hourly_pay ? `¥${project.hourly_pay}` : "-"}</TableCell>
                                <TableCell>
                                    <StatusBadge status={project.status as ProjectStatus} />
                                </TableCell>
                                <TableCell>{`${hours}時間${minutes}分`}</TableCell>
                                <TableCell>
                                    <WorkLogButton projectId={project.id} />
                                </TableCell>
                            </TableRow>
                        );
                    })}
                </TableBody>
            </Table>
        </div>
    );
};
