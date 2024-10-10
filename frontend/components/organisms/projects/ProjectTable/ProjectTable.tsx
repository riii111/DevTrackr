import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Project, ProjectStatus } from "@/types/project";
import { Badge } from "@/components/ui/badge";

// ステータスに応じた色を定義
const statusColors = {
    [ProjectStatus.Planning]: "bg-blue-100 text-blue-800 hover:bg-blue-100 hover:text-blue-800",
    [ProjectStatus.InProgress]: "bg-yellow-100 text-yellow-800 hover:bg-yellow-100 hover:text-yellow-800",
    [ProjectStatus.Completed]: "bg-green-100 text-green-800 hover:bg-green-100 hover:text-green-800",
    [ProjectStatus.OnHold]: "bg-gray-100 text-gray-800 hover:bg-gray-100 hover:text-gray-800",
    [ProjectStatus.Cancelled]: "bg-red-100 text-red-800 hover:bg-red-100 hover:text-red-800",
};

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
                    {/* <TableHead>会社ID</TableHead> */}
                    <TableHead>時給</TableHead>
                    <TableHead>ステータス</TableHead>
                    <TableHead>総作業時間</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                {projects.map((project) => (
                    <TableRow key={project.id && typeof project.id === 'object' ? project.id.$oid : project.id}>
                        <TableCell>{project.title}</TableCell>
                        <TableCell>{project.description || "-"}</TableCell>
                        <TableCell>{project.skill_labels?.join(", ") || "-"}</TableCell>
                        {/* <TableCell>{project.company_id && typeof project.company_id === 'object' ? project.company_id.$oid : project.company_id}</TableCell> */}
                        <TableCell>{project.hourly_pay ? `¥${project.hourly_pay}` : "-"}</TableCell>
                        <TableCell>
                            <Badge className={statusColors[project.status]}>
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