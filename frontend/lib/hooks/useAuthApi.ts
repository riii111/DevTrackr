import { customFetch } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";

const ENDPOINT = "/auth";

export function useAuthApi() {
  return {
    logout,
  };

  /**
   * ユーザーログアウト関数
   */
  async function logout(): Promise<void> {
    await customFetch(`${ENDPOINT}/logout/`, {
      method: "POST",
      credentials: "include",
    });
  }
}
