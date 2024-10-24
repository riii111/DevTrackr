import { customFetch } from "@/lib/api/core";
import {
  CreateProjectRequest,
  UpdateProjectRequest,
  CreateProjectResponse,
  ProjectResponse,
  ProjectsResponse,
} from "@/types/project";

const ENDPOINT = "/projects";

/**
 * プロジェクト一覧を取得する関数
 */
export async function getProjects(): Promise<ProjectsResponse> {
  const { data } = await customFetch<"GET", undefined, ProjectsResponse>(
    ENDPOINT,
    {
      method: "GET",
    }
  );
  return data;
}

/**
 * 対象のプロジェクトの詳細を取得する関数
 */
export async function getProjectById(id: string): Promise<ProjectResponse> {
  const { data } = await customFetch<"GET", undefined, ProjectResponse>(
    `${ENDPOINT}/${id}`,
    {
      method: "GET",
    }
  );
  return data;
}

/**
 * プロジェクトを作成する関数
 */
export async function createProject(
  projectData: CreateProjectRequest
): Promise<CreateProjectResponse> {
  const { data } = await customFetch<
    "POST",
    CreateProjectRequest,
    CreateProjectResponse
  >(ENDPOINT, {
    method: "POST",
    body: projectData,
  });
  return data;
}

/**
 * プロジェクトを更新する関数
 */
export async function updateProject(
  id: string,
  projectData: UpdateProjectRequest
): Promise<void> {
  await customFetch<"PUT", UpdateProjectRequest, void>(`${ENDPOINT}/${id}/`, {
    method: "PUT",
    body: projectData,
  });
}
