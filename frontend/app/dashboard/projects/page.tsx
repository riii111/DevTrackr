import ProjectsContent from "@/app/dashboard/projects/ProjectsContent";
import ProjectsClientComponents from "@/app/dashboard/projects/ProjectsClientComponents";
import { useCompaniesApi } from "@/lib/hooks/useCompaniesApi";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function ProjectListPage() {
    const { getCompanies } = useCompaniesApi();
    const companies = await getCompanies();

    return (
        <>
            <ProjectsClientComponents companies={companies} />
            <ProjectsContent bgColor={bgColor} />
        </>
    );
}