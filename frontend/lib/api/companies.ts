import { fetchApi } from "./core";
// import { CompanyView } from "../types/api";

export async function getCompanies(): Promise<CompanyView[]> {
  return fetchApi("/companies");
}

export async function getCompany(id: string): Promise<CompanyView> {
  return fetchApi(`/companies/${id}`);
}
