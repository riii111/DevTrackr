import { fetchApi } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";

const AUTH_ENDPOINT = "/auth";

export function useAuthApi() {
  return {
    login,
    logout,
    register,
    refreshAccessToken,
  };

  /**
   * ユーザーログイン関数
   */
  async function login(email: string, password: string): Promise<AuthResponse> {
    const response = await fetchApi<AuthResponse>(`${AUTH_ENDPOINT}/login`, {
      method: "POST",
      body: JSON.stringify({ email, password }),
    });
    return response.data;
  }

  /**
   * ユーザーログアウト関数
   */
  async function logout(): Promise<void> {
    await fetchApi(`${AUTH_ENDPOINT}/logout`, { method: "POST" });
  }

  /**
   * ユーザー登録関数
   */
  async function register(
    username: string,
    email: string,
    password: string
  ): Promise<AuthTokenCreatedResponse> {
    const response = await fetchApi<AuthTokenCreatedResponse>(
      `${AUTH_ENDPOINT}/register`,
      {
        method: "POST",
        body: JSON.stringify({ username, email, password }),
      }
    );
    return response.data;
  }

  /**
   * アクセストークンリフレッシュ関数
   */
  async function refreshAccessToken(): Promise<string> {
    return refreshAccessToken();
  }
}

// ミドルウェアやcore.tsで使用するために個別にエクスポート
export async function refreshAccessToken(): Promise<string> {
  const response = await fetchApi<{ access_token: string }>(
    `${AUTH_ENDPOINT}/refresh`,
    {
      method: "POST",
    }
  );
  return response.data.access_token;
}
