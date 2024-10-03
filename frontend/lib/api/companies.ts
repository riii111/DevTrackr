import { fetchApi } from "./core";
import { Company } from "@/types/company";

export async function getCompanies(): Promise<Company[]> {
  return fetchApi("/companies");
}

export async function getCompany(id: string): Promise<Company> {
  return fetchApi(`/companies/${id}`);
}
