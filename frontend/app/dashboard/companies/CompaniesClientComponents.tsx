"use client";

import dynamic from 'next/dynamic';
import { useDrawerStore } from "@/lib/store/useDrawerStore";
import { CompanyWithProjects } from "@/types/company";

const CompanyDrawer = dynamic(() => import("@/components/features/companies/Drawer/CompanyDrawer").then(mod => mod.CompanyDrawer), {
    ssr: false,
});

interface CompaniesClientComponentsProps {
    companiesWithProjects: CompanyWithProjects[];
}

export default function CompaniesClientComponents({ companiesWithProjects }: CompaniesClientComponentsProps) {
    const { drawerState } = useDrawerStore();

    return (
        <>
            {drawerState.main.isOpen && <CompanyDrawer />}
        </>
    );
}
