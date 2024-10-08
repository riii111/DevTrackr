import { customFetch } from "@/lib/api/core";
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
    const response = await customFetch<
      "POST",
      { email: string; password: string },
      AuthResponse
    >(`${AUTH_ENDPOINT}/login/`, {
      method: "POST",
      body: { email, password },
    });
    return response.data;
  }

  /**
   * ユーザーログアウト関数
   */
  async function logout(): Promise<void> {
    await customFetch(`${AUTH_ENDPOINT}/logout/`, { method: "POST" });
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
    >(`${AUTH_ENDPOINT}/register/`, {
      method: "POST",
      body: { username, email, password },
    });
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
export async function refreshAccessToken(headers: Headers): Promise<string> {
  const response = await customFetch<"POST", never, { access_token: string }>(
    `${AUTH_ENDPOINT}/refresh/`,
    {
      method: "POST",
      headers: headers,
    }
  );
  return response.data.access_token;
}
