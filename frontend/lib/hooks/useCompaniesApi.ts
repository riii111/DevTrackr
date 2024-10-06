import useSWR from "swr";
import { customFetch } from "@/lib/api/core";
import {
  Company,
  GetCompaniesParams,
  CreateCompanyRequest,
  UpdateCompanyRequest,
} from "@/types/company";
import { ApiResponse } from "@/types/api";

const ENDPOINT = "/companies/";

export function useCompaniesApi() {
  const {
    data: companies,
    error: companiesError,
    mutate: mutateCompanies,
  } = useSWR<ApiResponse<Company[]>>(ENDPOINT, (url: string) =>
    customFetch<"GET", GetCompaniesParams, Company[]>(url, { method: "GET" })
  );

  /**
   * 企業を作成する関数
   */
  return {
    createCompanyMutation,
    updateCompanyMutation,
    useCompany,
    companies: companies?.data,
    isLoading: !companiesError && !companies,
    isError: companiesError,
    mutateCompanies,
  };

  async function createCompanyMutation(
    companyData: CreateCompanyRequest
  ): Promise<Company> {
    const response = await customFetch<"POST", CreateCompanyRequest, Company>(
      `${ENDPOINT}`,
      {
        method: "POST",
        body: companyData,
      }
    );
    return response.data;
  }

  /**
   * 企業を更新する関数
   */
  async function updateCompanyMutation(
    id: string,
    companyData: UpdateCompanyRequest
  ): Promise<Company> {
    const response = await customFetch<"PUT", UpdateCompanyRequest, Company>(
      `${ENDPOINT}/${id}/`,
      {
        method: "PUT",
        body: companyData,
      }
    );
    return response.data;
  }

  /**
   * 企業を取得する関数
   */
  function useCompany(id: string) {
    const { data, error, mutate } = useSWR<ApiResponse<Company>>(
      `${ENDPOINT}/${id}/`,
      (url: string) =>
        customFetch<"GET", Record<string, never>, Company>(url, {
          method: "GET",
        })
    );
    return {
      company: data?.data,
      isLoading: !error && !data,
      isError: error,
      mutate,
    };
  }
}
