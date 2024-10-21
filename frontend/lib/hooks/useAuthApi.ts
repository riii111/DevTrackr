import { customFetch } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";

const ENDPOINT = "/auth";

export function useAuthApi() {
  return {
    login,
    logout,
    register,
  };

  /**
   * ユーザーログイン関数
   */
  async function login(email: string, password: string): Promise<AuthResponse> {
    const response = await customFetch<
      "POST",
      { email: string; password: string },
      AuthResponse
    >(`${ENDPOINT}/login/`, {
      method: "POST",
      body: { email, password },
      credentials: "include",
    });
    return response;
  }

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
