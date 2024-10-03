import { useState, useCallback } from "react";
import { fetchApi } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";

const AUTH_ENDPOINT = "/auth";

export function useAuthApi() {
  const login = useCallback(
    async (email: string, password: string): Promise<AuthResponse> => {
      const response = await fetchApi<AuthResponse>(`${AUTH_ENDPOINT}/login`, {
        method: "POST",
        body: JSON.stringify({ email, password }),
      });
      return response.data;
    },
    []
  );

  const logout = useCallback(async (): Promise<void> => {
    await fetchApi(`${AUTH_ENDPOINT}/logout`, { method: "POST" });
  }, []);

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
    register,
    refreshAccessToken,
  };
}
