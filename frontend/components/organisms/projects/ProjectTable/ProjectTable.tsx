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
import { SkillIcon } from "@/components/atoms/SkillIcon";
import { TruncatedText } from "@/components/atoms/TruncatedText";
import { StatusBadge } from "@/components/atoms/StatusBadge";

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
                    {projects.map((project) => (
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
                            <TableCell>{Math.floor(project.total_working_time / 3600)}時間</TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    );
};
