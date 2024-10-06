import useSWR from "swr";
import { fetchApi } from "@/lib/api/core";
import { Company } from "@/types/company";
import { ApiResponse } from "@/types/api";

const ENDPOINT = "/companies";

export function useCompaniesApi() {
  const {
    data: companies,
    error: companiesError,
    mutate: mutateCompanies,
  } = useSWR<ApiResponse<Company[]>>(ENDPOINT, fetchApi);

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
    companyData: Partial<Company>
  ): Promise<Company> {
    const response = await fetchApi<Company>(`${ENDPOINT}`, {
      method: "POST",
      body: JSON.stringify(companyData),
    });
    return response.data;
  }

  /**
   * 企業を更新する関数
   */
  async function updateCompanyMutation(
    id: string,
    companyData: Partial<Company>
  ): Promise<Company> {
    const response = await fetchApi<Company>(`${ENDPOINT}/${id}`, {
      method: "PUT",
      body: JSON.stringify(companyData),
    });
    return response.data;
  }

  /**
   * 企業を取得する関数
   */
  function useCompany(id: string) {
    const { data, error, mutate } = useSWR<ApiResponse<Company>>(
      `${ENDPOINT}/${id}`,
      fetchApi
    );
    return {
      company: data?.data,
      isLoading: !error && !data,
      isError: error,
      mutate,
    };
  }
}
