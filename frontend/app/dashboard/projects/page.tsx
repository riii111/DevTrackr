import ProjectsContent from "@/app/dashboard/projects/ProjectsContent";
import ProjectsClientComponents from "@/app/dashboard/projects/ProjectsClientComponents";
import { useCompaniesApi } from "@/lib/hooks/useCompaniesApi";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function ProjectListPage() {
    const { getCompaniesWithProjects } = useCompaniesApi();
    const { companies } = await getCompaniesWithProjects();

    return (
        <>
            <ProjectsClientComponents companiesWithProjects={companies} />
            <ProjectsContent bgColor={bgColor} />
        </>
    );
}