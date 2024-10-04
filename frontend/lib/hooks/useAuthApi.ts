import { useState, useCallback } from "react";
import { fetchApi } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";
import { ApiResponse } from "@/types/api";

const AUTH_ENDPOINT = "/auth";

export function useAuthApi() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  /**
   * ユーザーログイン関数
   */
  const login = useCallback(
    async (email: string, password: string): Promise<AuthResponse> => {
      const response = await fetchApi<AuthResponse>(`${AUTH_ENDPOINT}/login`, {
        method: "POST",
        body: JSON.stringify({ email, password }),
      });

      if (response.data.access_token && response.data.refresh_token) {
        localStorage.setItem('access_token', response.data.access_token);
        localStorage.setItem('refresh_token', response.data.refresh_token);
        setIsAuthenticated(true);
      }

      return response.data;
    },
    []
  );

  // Cookieを取得するヘルパー関数
  function getCookie(name: string): string | null {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop()?.split(';').shift() || null;
    return null;
  }

  /**
   * ユーザーログアウト関数
   */
  const logout = useCallback(async (): Promise<void> => {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    setIsAuthenticated(false);
    await fetchApi(`${AUTH_ENDPOINT}/logout`, { method: "POST" });
  }, []);

  /**
   * ユーザー登録関数
   */
  const register = useCallback(
    async (
      username: string,
      email: string,
      password: string
    ): Promise<AuthTokenCreatedResponse> => {
      const response = await fetchApi<AuthTokenCreatedResponse>(
        `${AUTH_ENDPOINT}/register`,
        {
          method: "POST",
          body: JSON.stringify({ username, email, password }),
        }
      );
      return response.data;
    },
    []
  );

  /**
   * アクセストークンリフレッシュ関数
   */
  const refreshAccessToken = useCallback(async (): Promise<string> => {
    const response = await fetchApi<{ access_token: string }>(
      `${AUTH_ENDPOINT}/refresh`,
      {
        method: "POST",
      }
    );
    return response.data.access_token;
  }, []);

  return {
    login,
    logout,
    isAuthenticated,
    register,
    refreshAccessToken,
  };
}
