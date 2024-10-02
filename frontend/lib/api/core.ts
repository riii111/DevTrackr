// lib/api/core.ts
import { getSession } from "next-auth/react";
import { toast } from "react-toastify";

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

async function refreshAccessToken() {
  // TODO: リフレッシュトークンを使用してアクセストークンを更新する処理
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
  });

  if (response.status === HTTP_STATUS.UNAUTHORIZED) {
    // 401エラーの場合、アクセストークンの更新を試みる
    await refreshAccessToken();
    // 更新されたトークンで再度リクエストを行う
    response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      headers,
    });
  }

  if (!response.ok) {
    let errorMessage = "An error occurred";
    try {
      const errorData = await response.json();
      errorMessage = errorData.message || errorMessage;
    } catch (e) {
      // JSON解析に失敗した場合は、デフォルトのエラーメッセージを使用
    }

    if (response.status === HTTP_STATUS.UNAUTHORIZED) {
      // ログイン情報の有効期限切れの場合、スナックバーを表示
      toast.error(
        "ログイン情報の有効期限が切れました。再度ログインしてください。"
      );
    }

    throw new ApiError(response.status, errorMessage);
  }

  return response.json();
}

// 注: Next.jsではnext-authがCookieの管理を行うため、
// このファイルではCookieの設定や取得は行わない
