import ProjectsClientComponents from "@/app/dashboard/projects/ProjectsClientComponents";
import { getCompaniesWithProjects } from "@/lib/api/companies";
import { getProjects } from "@/lib/api/projects";
import { ProjectTable } from "@/components/features/projects/Table/ProjectTable";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function ProjectListPage() {
    const [{ companies }, projects] = await Promise.all([
        getCompaniesWithProjects(),
        getProjects()
    ]);

    return (
        <>
            <ProjectsClientComponents companiesWithProjects={companies} />
            <div className={`p-6 rounded-lg ${bgColor} text-text-primary`}>
                <h1 className="text-2xl font-bold mb-4">プロジェクト一覧</h1>
                <ProjectTable projects={projects} />
            </div>
        </>
    );
}