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

  /**
   * ユーザー登録関数
   */
  async function register(
    username: string,
    email: string,
    password: string
  ): Promise<AuthTokenCreatedResponse> {
    const response = await customFetch<
      "POST",
      { username: string; email: string; password: string },
      AuthTokenCreatedResponse
    >(`${ENDPOINT}/register/`, {
      method: "POST",
      body: { username, email, password },
      credentials: "include",
    });
    return response;
  }
}
