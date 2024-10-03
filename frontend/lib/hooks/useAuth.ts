import { fetchApi } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";

const AUTH_ENDPOINT = "/auth";

export function useAuthApi() {
  const loginMutation = async (
    email: string,
    password: string
  ): Promise<AuthResponse> => {
    const response = await fetchApi<AuthResponse>(`${AUTH_ENDPOINT}/login`, {
      method: "POST",
      body: JSON.stringify({ email, password }),
    });
    return response.data;
  };

  const logoutMutation = async (): Promise<void> => {
    await fetchApi(`${AUTH_ENDPOINT}/logout`, { method: "POST" });
  };

  const registerMutation = async (
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
  };

  // TODO: "/me"エンドポイントが実装されるまでコメントアウトまたは削除
  // const {
  //   data: currentUser,
  //   error: currentUserError,
  //   mutate: mutateCurrentUser,
  // } = useSWR<ApiResponse<User>>(`${AUTH_ENDPOINT}/me`, fetchApi);

  return {
    loginMutation,
    logoutMutation,
    registerMutation,
    // currentUser: currentUser?.data,
    // isLoading: !currentUserError && !currentUser,
    // isError: currentUserError,
    // mutateCurrentUser,
  };
}
