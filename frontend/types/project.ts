export interface Project {
  id: string;
  title: string;
  description?: string;
  skill_labels?: string[];
  company_id: string;
  hourly_pay?: number;
  status: ProjectStatus;
  total_working_time: number;
  created_at: string;
  updated_at?: string;
}

export enum ProjectStatus {
  Planning = "Planning",
  InProgress = "InProgress",
  Completed = "Completed",
  OnHold = "OnHold",
  Cancelled = "Cancelled",
}

export interface GetProjectsParams {
  title?: string;
  status?: ProjectStatus;
  skill_labels?: string[];
  company_id?: string;
  limit?: number;
  offset?: number;
  sort?: string[];
}

export interface CreateProjectRequest {
  title: string;
  description?: string;
  skill_labels?: string[];
  company_id: string;
  hourly_pay?: number;
  status: ProjectStatus;
}

export interface UpdateProjectRequest {
  title: string;
  description?: string;
  skill_labels?: string[];
  company_id: string;
  hourly_pay?: number;
  status: ProjectStatus;
  total_working_time: number;
}

export interface ProjectsResponse {
  projects: Project[];
  // total: number;
}

export interface ProjectResponse {
  project: Project;
}

export interface CreateProjectResponse {
  id: string;
}
