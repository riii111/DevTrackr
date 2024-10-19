import { customFetch } from "@/lib/api/core";
import {
  Company,
  CreateCompanyRequest,
  UpdateCompanyRequest,
  CompanyResponse,
  CompaniesResponse,
  CompaniesWithProjectsResponse,
  CreateCompanyResponse,
} from "@/types/company";

const ENDPOINT = "/companies/";

export function useCompaniesApi() {
  return {
    getCompanies,
    createCompany,
    updateCompany,
    getCompanyById,
    getCompaniesWithProjects,
  };

  /**
   * 企業一覧を取得する関数
   */
  async function getCompanies(): Promise<CompaniesResponse> {
    const response = await customFetch<"GET", undefined, CompaniesResponse>(
      ENDPOINT,
      {
        method: "GET",
      }
    );
    return response;
  }

  /**
   * 企業一覧（プロジェクトを含む）を取得する関数
   */
  async function getCompaniesWithProjects(): Promise<CompaniesWithProjectsResponse> {
    const response = await customFetch<
      "GET",
      undefined,
      CompaniesWithProjectsResponse
    >(`${ENDPOINT}with-projects/`, {
      method: "GET",
    });
    return response;
  }

  /**
   * 企業を作成する
   */
  async function createCompany(
    companyData: CreateCompanyRequest
  ): Promise<CreateCompanyResponse> {
    const response = await customFetch<
      "POST",
      CreateCompanyRequest,
      CreateCompanyResponse
    >(ENDPOINT, {
      method: "POST",
      body: companyData,
    });
    return response;
  }

  /**
   * 企業を更新する関数
   */
  async function updateCompany(
    id: string,
    companyData: UpdateCompanyRequest
  ): Promise<void> {
    await customFetch<"PUT", UpdateCompanyRequest, void>(`${ENDPOINT}/${id}/`, {
      method: "PUT",
      body: companyData,
    });
  }

  /**
   * 特定の企業を取得する関数
   */
  async function getCompanyById(id: string): Promise<CompanyResponse> {
    const response = await customFetch<
      "GET",
      Record<string, never>,
      CompanyResponse
    >(`${ENDPOINT}/${id}/`, {
      method: "GET",
    });
    return response;
  }
}
