import useSWR from "swr";
import { fetchApi } from "@/lib/api/core";
import { Company } from "@/types/company";
import { ApiResponse } from "@/types/api";

export function useCompaniesApi() {
  const endpoint = "/companies";

  /**
   * 企業を作成する関数
   */
  const createCompanyMutation = async (companyData: Partial<Company>) => {
    const response = await fetchApi<Company>(`${endpoint}`, {
      method: "POST",
      body: JSON.stringify(companyData),
    });
    return response.data;
  };

  /**
   * 企業を更新する関数
   */
  const updateCompanyMutation = async (
    id: string,
    companyData: Partial<Company>
  ) => {
    const response = await fetchApi<Company>(`${endpoint}/${id}`, {
      method: "PUT",
      body: JSON.stringify(companyData),
    });
    return response.data;
  };

  /**
   * 企業を取得する関数
   */
  const {
    data: companies,
    error: companiesError,
    mutate: mutateCompanies,
  } = useSWR<ApiResponse<Company[]>>(endpoint, fetchApi);

  const useCompany = (id: string) => {
    const { data, error, mutate } = useSWR<ApiResponse<Company>>(
      `${endpoint}/${id}`,
      fetchApi
    );
    return {
      company: data?.data,
      isLoading: !error && !data,
      isError: error,
      mutate,
    };
  };

  return {
    createCompanyMutation,
    updateCompanyMutation,
    useCompany,
    companies: companies?.data,
    isLoading: !companiesError && !companies,
    isError: companiesError,
    mutateCompanies,
  };
}
