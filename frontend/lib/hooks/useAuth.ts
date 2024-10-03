import useSWR from "swr";
import { fetchApi } from "@/lib/api/core";
import { User } from "@/types/user";
import { ApiResponse } from "@/types/api";

export function useAuthApi() {
  const endpoint = "/auth";

  const loginMutation = async (email: string, password: string) => {
    const response = await fetchApi<User>(`${endpoint}/login`, {
      method: "POST",
      body: JSON.stringify({ email, password }),
    });
    return response.data;
  };

  const logoutMutation = async () => {
    await fetchApi(`${endpoint}/logout`, { method: "POST" });
  };

  const registerMutation = async (
    username: string,
    email: string,
    password: string
  ) => {
    const response = await fetchApi<User>(`${endpoint}/register`, {
      method: "POST",
      body: JSON.stringify({ username, email, password }),
    });
    return response.data;
  };

  const {
    data: currentUser,
    error: currentUserError,
    mutate: mutateCurrentUser,
  } = useSWR<ApiResponse<User>>(`${endpoint}/me`, fetchApi);

  const refreshMutation = async () => {
    const response = await fetchApi<User>(`${endpoint}/token/refresh`, {
      method: "POST",
    });
    if (response.data) {
      await mutateCurrentUser({ data: response.data });
    }
    return response.data;
  };

  return {
    loginMutation,
    logoutMutation,
    registerMutation,
    refreshMutation,
    currentUser: currentUser?.data,
    isLoading: !currentUserError && !currentUser,
    isError: currentUserError,
    mutateCurrentUser,
  };
}
