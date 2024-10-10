import { Project } from "@/types/project";

export interface Company {
  id: string;
  company_name: string;
  establishment_year: number;
  location: string;
  website_url: string;
  employee_count: number;
  annual_sales?: AnnualSales;
  affiliation_start_date: string;
  affiliation_end_date?: string;
  contract_type: ContractType;
  major_clients?: string[];
  major_services?: string[];
  average_hourly_rate?: number;
  bonus?: Bonus;
  status: CompanyStatus;
  created_at: string;
  updated_at?: string;
}

export interface AnnualSales {
  amount: number;
  fiscal_year: number;
}

export enum ContractType {
  FullTime = "FullTime",
  PartTime = "PartTime",
  Contract = "Contract",
  Freelance = "Freelance",
  SideJob = "SideJob",
}

export enum CompanyStatus {
  PendingContract = "PendingContract",
  Contract = "Contract",
  Completed = "Completed",
  Cancelled = "Cancelled",
}

export interface Bonus {
  amount: number;
  frequency: number;
}

export interface CompanyWithProjects extends Company {
  projects: Project[];
}

// リクエスト
export interface CreateCompanyRequest {
  company_name: string;
  establishment_year: number;
  location: string;
  website_url: string;
  employee_count: number;
  annual_sales?: AnnualSales;
  contract_type: ContractType;
  major_clients?: string[];
  major_services?: string[];
  average_hourly_rate?: number;
  bonus?: Bonus;
  status: CompanyStatus;
  affiliation_start_date: string;
  affiliation_end_date?: string;
}

export interface UpdateCompanyRequest {
  company_name: string;
  establishment_year: number;
  location: string;
  website_url: string;
  employee_count: number;
  annual_sales?: AnnualSales;
  contract_type: ContractType;
  major_clients?: string[];
  major_services?: string[];
  average_hourly_rate?: number;
  bonus?: Bonus;
  status: CompanyStatus;
  affiliation_start_date: string;
  affiliation_end_date?: string;
}

// レスポンス
export interface CompaniesWithProjectsResponse {
  companies: CompanyWithProjects[];
  total: number;
}

export interface CreateCompanyResponse {
  id: string;
}

export interface CompanyResponse {
  company: Company;
}

export interface CompaniesResponse {
  companies: Company[];
  // total: number;
}
