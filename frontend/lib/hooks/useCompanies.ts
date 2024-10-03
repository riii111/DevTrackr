import useSWR from "swr";
import { fetchApi } from "@/lib/api/core";
import { Company } from "@/types/company";

export function useCompanies() {
  const { data, error, mutate } = useSWR<Company[]>("/companies", fetchApi);

  return {
    companies: data,
    isLoading: !error && !data,
    isError: error,
    mutate,
  };
}

export function useCompany(id: string) {
  const { data, error, mutate } = useSWR<Company>(`/companies/${id}`, fetchApi);

  return {
    company: data,
    isLoading: !error && !data,
    isError: error,
    mutate,
  };
}
