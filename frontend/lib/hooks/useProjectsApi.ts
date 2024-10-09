import { customFetch } from "@/lib/api/core";
import {
  Project,
  CreateProjectRequest,
  UpdateProjectRequest,
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
  async function getProjects(): Promise<Project[]> {
    const response = await customFetch<"GET", Record<string, never>, Project[]>(
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
  ): Promise<{ id: string }> {
    const response = await customFetch<
      "POST",
      CreateProjectRequest,
      { id: string }
    >(ENDPOINT, {
      method: "POST",
      body: projectData,
    });
    // TODO: キャッシュ更新すべき？あとで要考慮
    return response;
  }

  /**
   * プロジェクトを更新する関数
   */
  async function updateProject(
    id: string,
    projectData: UpdateProjectRequest
  ): Promise<Project> {
    const response = await customFetch<"PUT", UpdateProjectRequest, Project>(
      `${ENDPOINT}${id}/`,
      {
        method: "PUT",
        body: projectData,
      }
    );
    // TODO: キャッシュ更新すべき？あとで要考慮
    return response;
  }
}
