"use client"

import React, { useMemo, useRef, useCallback } from "react"
import { useDrawerStore } from "@/lib/store/useDrawerStore"
import { CompanyDrawerToolbar } from "@/components/features/companies/Drawer/content/CompanyDrawerToolbar"
import { getCompanyById } from "@/lib/api/companies";
import useSWR from "swr";
import { ErrorAlert } from "@/components/core/ErrorAlert"
import { CompanyDetails } from "@/components/features/companies/Drawer/content/CompanyDetails"
import { LoadingSkeleton } from "@/components/core/LoadingSkeleton"

interface Props {
    width?: number
    drawerType: "main" | "sub"
    selectedCompanyId?: string | null
}

function useCompanyDetails(companyId: string | null) {
    console.log("companyId", companyId)

    const fetchCompany = useCallback(() => {
        return companyId ? getCompanyById(companyId) : null;
    }, [companyId]);

    const { data, error, isLoading } = useSWR(
        companyId ? `company-${companyId}` : null,
        fetchCompany,
        { revalidateOnFocus: false }
    );

    return {
        company: data,
        isLoading,
        error
    };
}

export const CompanyDrawerBody: React.FC<Props> = React.memo(({ width, drawerType, selectedCompanyId }) => {
    const drawerStore = useDrawerStore()
    const subDrawer = useRef<HTMLDivElement>(null)

    const state = drawerStore.drawerState[drawerType]
    const isSubDrawer = drawerType == "sub"

    const drawerStyle = useMemo(() => {
        if (isSubDrawer) {
            return {
                width: `${state.isOpen ? width : 0}px`
            }
        }
        return undefined
    }, [isSubDrawer, state.isOpen, width])

    const { company, isLoading, error } = useCompanyDetails(selectedCompanyId ?? null);

    const handleSave = (updatedCompany: any) => {
        // TODO: PUTリクエストを送信
        console.log("Updated company:", updatedCompany);
    };

    return (
        <div
            ref={isSubDrawer ? subDrawer : undefined}
            style={drawerStyle}
            className={`flex flex-col min-h-screen ${isSubDrawer ? 'shadow-inner transition-all duration-300' : ''}`}
        >
            <CompanyDrawerToolbar drawerType={drawerType} />
            <hr className="border-gray-300" />
            <div className="p-4">
                {isLoading && <LoadingSkeleton />}
                {error && <ErrorAlert error={error} />}
                {company && <CompanyDetails company={company} onSave={handleSave} />}
            </div>
        </div>
    )
});

CompanyDrawerBody.displayName = "CompanyDrawerBody"