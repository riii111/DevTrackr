import { customFetch } from "@/lib/api/core";
import { UpdateUserRequest, UserResponse } from "@/types/user";

const ENDPOINT = "/users";

export function useUserApi() {
  return {
    getCurrentUserDetails,
    updateUser,
  };

  /**
   * ログイン中のユーザーの詳細を取得する関数
   */
  async function getCurrentUserDetails(): Promise<UserResponse> {
    const response = await customFetch<"GET", undefined, UserResponse>(
      `${ENDPOINT}/me`,
      {
        method: "GET",
      }
    );
    return response;
  }

  /**
   * ユーザーを更新する関数
   */
  async function updateUser(
    id: string,
    userData: UpdateUserRequest
  ): Promise<void> {
    await customFetch<"PUT", UpdateUserRequest, void>(`${ENDPOINT}/${id}/`, {
      method: "PUT",
      body: userData,
    });
  }
}
