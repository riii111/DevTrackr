import { useCompaniesApi } from "@/lib/hooks/useCompaniesApi";
import { CompanyTable } from "@/components/organisms/companies/Table/CompanyTable";
import CompaniesClientComponents from "@/app/dashboard/companies/CompaniesClientComponents";

const bgColor = "bg-main-translucent backdrop-filter backdrop-blur-sm";

export default async function CompaniesPage() {
    const { getCompaniesWithProjects } = useCompaniesApi();

    const [{ companies }] = await Promise.all([
        getCompaniesWithProjects()
    ]);

    return (
        <>
            <CompaniesClientComponents companiesWithProjects={companies} />
            <div className={`p-6 rounded-lg ${bgColor} text-text-primary`}>
                <h1 className="text-2xl font-bold mb-4">企業一覧</h1>
                <CompanyTable companies={companies} />
            </div>
        </>
    );
}