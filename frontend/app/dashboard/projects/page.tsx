import ProjectsClientComponents from "@/app/dashboard/projects/ProjectsClientComponents";
import { useCompaniesApi } from "@/lib/hooks/useCompaniesApi";
import { useProjectsApi } from "@/lib/hooks/useProjectsApi";
import { ProjectTable } from "@/components/organisms/projects/ProjectTable/ProjectTable";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function ProjectListPage() {
    const { getCompaniesWithProjects } = useCompaniesApi();
    const { getProjects } = useProjectsApi();
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