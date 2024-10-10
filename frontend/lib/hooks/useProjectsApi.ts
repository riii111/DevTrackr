import { customFetch } from "@/lib/api/core";
import {
  Project,
  CreateProjectRequest,
  UpdateProjectRequest,
  ProjectsResponse,
  CreateProjectResponse,
  ProjectResponse,
} from "@/types/project";

const ENDPOINT = "/projects/";

export function useProjectsApi() {
  return {
    getProjects,
    createProject,
    updateProject,
  };

  /**
   * プロジェクト一覧を取得する関数
   */
  async function getProjects(): Promise<ProjectResponse> {
    const response = await customFetch<"GET", undefined, ProjectResponse>(
      ENDPOINT,
      {
        method: "GET",
      }
    );
    return response;
  }

  /**
   * プロジェクトを作成する関数
   */
  async function createProject(
    projectData: CreateProjectRequest
  ): Promise<CreateProjectResponse> {
    const response = await customFetch<
      "POST",
      CreateProjectRequest,
      CreateProjectResponse
    >(ENDPOINT, {
      method: "POST",
      body: projectData,
    });
    return response;
  }

  /**
   * プロジェクトを更新する関数
   */
  async function updateProject(
    id: string,
    projectData: UpdateProjectRequest
  ): Promise<void> {
    await customFetch<"PUT", UpdateProjectRequest, void>(`${ENDPOINT}/${id}/`, {
      method: "PUT",
      body: projectData,
    });
  }
}
