import { getSession } from "next-auth/react";
import { toast } from "react-toastify";
import { refreshAccessToken } from "./auth";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

const HTTP_STATUS = {
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  TOO_MANY_REQUESTS: 429,
  INTERNAL_SERVER_ERROR: 500,
};

class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

export async function fetchApi(endpoint: string, options: RequestInit = {}) {
  const session = await getSession();
  const headers = new Headers(options.headers);

  if (session?.accessToken) {
    headers.set("Authorization", `Bearer ${session.accessToken}`);
  }

  let response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers,
    credentials: "include", // Cookieを送信
  });

  if (response.status === HTTP_STATUS.UNAUTHORIZED) {
    try {
      const newAccessToken = await refreshAccessToken();
      headers.set("Authorization", `Bearer ${newAccessToken}`);
      response = await fetch(`${API_BASE_URL}${endpoint}`, {
        ...options,
        headers,
        credentials: "include",
      });
    } catch (error) {
      toast.error(
        "ログイン情報の有効期限が切れました。再度ログインしてください。"
      );
      throw new ApiError(HTTP_STATUS.UNAUTHORIZED, "Session expired");
    }
  }

  if (!response.ok) {
    let errorMessage = "An error occurred";
    try {
      const errorData = await response.json();
      errorMessage = errorData.message || errorMessage;
    } catch (e) {
      // JSON解析に失敗した場合は、デフォルトのエラーメッセージを使用
    }
    throw new ApiError(response.status, errorMessage);
  }

  return response.json();
}
