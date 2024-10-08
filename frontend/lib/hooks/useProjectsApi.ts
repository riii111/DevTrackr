import useSWR from "swr";
import { customFetch } from "@/lib/api/core";
import {
  Project,
  CreateProjectRequest,
  UpdateProjectRequest,
} from "@/types/project";
import { ApiResponse } from "@/types/api";

const ENDPOINT = "/projects/";

export function useProjectsApi() {
  const { data, error, mutate } = useSWR<ApiResponse<Project[]>>(
    ENDPOINT,
    (url: string) =>
      customFetch<"GET", Record<string, never>, Project[]>(url, {
        method: "GET",
      }),
    {
      revalidateOnFocus: false,
    }
  );

  /**
   * プロジェクトを作成する関数
   */
  async function createProjectMutation(
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
    // await mutateProjects();
    return response.data;
  }

  /**
   * プロジェクトを更新する関数
   */
  async function updateProjectMutation(
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
    // await mutateProjects();
    return response.data;
  }

  /**
   * プロジェクトを取得する関数
   */
  function useProject(id: string) {
    const { data, error, mutate } = useSWR<ApiResponse<Project>>(
      `${ENDPOINT}${id}/`,
      (url: string) =>
        customFetch<"GET", Record<string, never>, Project>(url, {
          method: "GET",
        })
    );
    return {
      project: data?.data,
      isLoading: !error && !data,
      isError: error,
      mutate,
    };
  }

  return {
    projects: data?.data,
    isLoading: !error && !data,
    isError: error,
    mutateProjects: mutate,
    createProjectMutation,
    updateProjectMutation,
    useProject,
  };
}

// サーバーサイドでプロジェクトを取得するための関数
export async function getServerSideProjects(): Promise<ApiResponse<Project[]>> {
  return customFetch<"GET", Record<string, never>, Project[]>(ENDPOINT, {
    method: "GET",
  });
}
