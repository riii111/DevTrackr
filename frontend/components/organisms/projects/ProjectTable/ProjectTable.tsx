import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Project } from "@/types/project";

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
                    <TableHead>スキルラベル</TableHead>
                    <TableHead>会社ID</TableHead>
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
                        <TableCell>{project.company_id && typeof project.company_id === 'object' ? project.company_id.$oid : project.company_id}</TableCell>
                        <TableCell>{project.hourly_pay ? `¥${project.hourly_pay}` : "-"}</TableCell>
                        <TableCell>{project.status}</TableCell>
                        <TableCell>{project.total_working_time}時間</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    );
};