import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { MdChevronRight } from 'react-icons/md';
import Link from 'next/link';
import { Project } from "@/types/project";
import { statusColors } from "@/lib/constants/ProjectStatusColors";


interface ProjectTableProps {
    projects: Project[];
}

export const ProjectTable: React.FC<ProjectTableProps> = ({ projects }) => {
    if (!projects || projects.length === 0) {
        return <p>プロジェクトがありません。</p>;
    }

    return (
        <Table>
            <TableHeader>
                <TableRow>
                    <TableHead>タイトル</TableHead>
                    <TableHead>説明</TableHead>
                    <TableHead>技術スタック</TableHead>
                    <TableHead>時給</TableHead>
                    <TableHead>ステータス</TableHead>
                    <TableHead>総作業時間</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {projects.map((project) => (
                    <TableRow
                        key={project.id && typeof project.id === 'object' ? project.id.$oid : project.id}
                        className="group"
                    >
                        <TableCell className="relative p-0">
                            <Link
                                href={`/dashboard/projects?projectId=${project.id && typeof project.id === 'object' ? project.id.$oid : project.id}`}
                                className="flex items-center w-full h-full p-4 text-gray-900 hover:text-blue-600 transition-colors font-semibold"
                            >
                                <span>{project.title}</span>
                                <MdChevronRight className="ml-2 h-6 w-6 text-gray-500 group-hover:text-blue-600" />
                            </Link>
                        </TableCell>
                        <TableCell>{project.description || "-"}</TableCell>
                        <TableCell>{project.skill_labels?.join(", ") || "-"}</TableCell>
                        <TableCell>{project.hourly_pay ? `¥${project.hourly_pay}` : "-"}</TableCell>
                        <TableCell>
                            <Badge className={statusColors[project.status as keyof typeof statusColors]}>
                                {project.status}
                            </Badge>
                        </TableCell>
                        <TableCell>{project.total_working_time}時間</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    );
};