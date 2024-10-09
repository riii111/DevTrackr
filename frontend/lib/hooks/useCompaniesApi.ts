import { customFetch } from "@/lib/api/core";
import {
  Company,
  GetCompaniesParams,
  CreateCompanyRequest,
  UpdateCompanyRequest,
} from "@/types/company";

const ENDPOINT = "/companies/";

export function useCompaniesApi() {
  return {
    getCompanies,
    createCompany,
    updateCompany,
    getCompany,
  };

  /**
   * 企業一覧を取得する関数
   */
  async function getCompanies(): Promise<Company[]> {
    const response = await customFetch<"GET", GetCompaniesParams, Company[]>(
      ENDPOINT,
      {
        method: "GET",
      }
    );
    return response;
  }

  /**
   * 企業を作成する関数
   */
  async function createCompany(
    companyData: CreateCompanyRequest
  ): Promise<Company> {
    const response = await customFetch<"POST", CreateCompanyRequest, Company>(
      ENDPOINT,
      {
        method: "POST",
        body: companyData,
      }
    );
    return response;
  }

  /**
   * 企業を更新する関数
   */
  async function updateCompany(
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
    return response;
  }

  /**
   * 特定の企業を取得する関数
   */
  async function getCompany(id: string): Promise<Company> {
    const response = await customFetch<"GET", Record<string, never>, Company>(
      `${ENDPOINT}${id}/`,
      {
        method: "GET",
      }
    );
    return response;
  }
}
