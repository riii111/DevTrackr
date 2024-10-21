import { toast } from "@/lib/hooks/use-toast";
import { getClientSideAuthHeader } from "@/lib/utils/cookies";
import { redirect } from "next/navigation";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

// GETメソッドの場合はクエリパラメータ、それ以外はリクエストボディの型を定義
type FetchParams<T, M extends HttpMethod> = M extends "GET"
  ? T
  : Record<string, string | number | boolean>;
type FetchBody<T, M extends HttpMethod> = M extends
  | "POST"
  | "PUT"
  | "DELETE"
  | "PATCH"
  ? T | FormData
  : never;

// フェッチオプションのインターフェース定義
interface IFetchOptions<T extends Record<string, any>, M extends HttpMethod> {
  headers?: Record<string, any>;
  method: M;
  credentials?: RequestCredentials;
  params?: FetchParams<T, M>;
  body?: FetchBody<T, M>;
  cache?: RequestCache;
}

// APIエラーを表すカスタムエラークラス
export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
    this.name = "ApiError";
  }
}

// カスタムフェッチ関数
export async function customFetch<
  M extends HttpMethod,
  RequestInput extends Record<string, any> | undefined = Record<string, any>,
  RequestResult = unknown
>(
  endpoint: string,
  {
    headers: optionHeaders,
    credentials,
    method,
    body,
    params,
    cache = "no-cache",
  }: IFetchOptions<
    RequestInput extends Record<string, any>
      ? RequestInput
      : Record<string, never>,
    M
  >
) {
  if (!API_BASE_URL) {
    throw new Error("API_BASE_URL is not defined");
  }

  // 末尾にスラッシュ追加する事でプロキシ・バックエンドなどのサーバー側のリダイレクトを回避し、パフォーマンスを僅かに向上させる
  let url = `${API_BASE_URL}${endpoint}${endpoint.endsWith("/") ? "" : "/"}`;

  async function getAuthHeader(): Promise<Record<string, string>> {
    if (typeof window === "undefined") {
      const { getServerSideAuthHeader } = await import(
        "@/lib/utils/cookiesForServer"
      );
      return getServerSideAuthHeader();
    } else {
      return getClientSideAuthHeader();
    }
  }

  let authHeader = await getAuthHeader();
  let headers = new Headers({
    ...optionHeaders,
    ...authHeader,
  });

  if (!(body instanceof FormData) && !headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  const fetchOptions: RequestInit = {
    method,
    headers,
    cache,
    mode: "cors",
    credentials,
  };

  if (body) {
    fetchOptions.body = body instanceof FormData ? body : JSON.stringify(body);
  }

  // GETメソッドの場合、クエリパラメータをURLに追加
  if (params && method === "GET") {
    const searchParams = new URLSearchParams(params as Record<string, string>);
    url += `?${searchParams.toString()}`;
  }

  async function performFetch(): Promise<Response> {
    const response = await fetch(url, fetchOptions);
    if (response.status === 401) {
      // リフレッシュトークンを使用してアクセストークンを更新
      const refreshed = await refreshAccessToken();
      if (refreshed) {
        // 新しいアクセストークンでリクエストを再試行
        authHeader = await getAuthHeader();
        headers = new Headers({
          ...optionHeaders,
          ...authHeader,
        });
        fetchOptions.headers = headers;
        return fetch(url, fetchOptions);
      } else {
        throw new Error(
          "リフレッシュトークンが無効です。再度ログインしてください。"
        );
      }
    }
    return response;
  }

  try {
    const response = await performFetch();

    if (!response.ok) {
      const errorMessage = getErrorMessage(response.status);
      throw new ApiError(response.status, errorMessage);
    }

    return handleResponse(response);
  } catch (error) {
    handleFetchError(error);
    if (
      error instanceof Error &&
      error.message ===
        "リフレッシュトークンが無効です。再度ログインしてください。"
    ) {
      if (typeof window !== "undefined") {
        window.location.href = "/auth";
      } else {
        redirect("/auth");
      }
    }
    throw error;
  }
}

async function refreshAccessToken(): Promise<boolean> {
  // 循環参照を防ぐため、customFetchは使わない
  try {
    console.log("Refreshing access token...");
    console.log("Cookies being sent:", document.cookie);
    const response = await fetch(`${API_BASE_URL}/auth/refresh/`, {
      method: "POST",
      credentials: "include",
    });
    console.log("Refresh response status:", response.status);
    console.log("Refresh response headers:", response.headers);
    return response.ok;
  } catch (error) {
    console.error("Token refresh error:", error);
    return false;
  }
}

async function handleResponse(response: Response): Promise<any> {
  if (response.status === 204) {
    return {};
  }

  const contentType = response.headers.get("content-type");
  if (contentType && contentType.includes("application/json")) {
    return response.json();
  }

  return {};
}

function handleFetchError(error: unknown) {
  if (error instanceof ApiError) {
    const message = `${error.status}: ${error.message}`;
    if (typeof window !== "undefined") {
      // クライアントサイドの場合はtoastを使用
      toast({
        variant: "error",
        title: "エラー",
        description: message,
      });
    } else {
      // サーバーサイドの場合はコンソールにエラーを出力
      console.error("API Error:", message);
    }
  } else {
    const message = "予期せぬエラーが発生しました。";
    if (typeof window !== "undefined") {
      // クライアントサイドの場合はtoastを使用
      toast({
        variant: "error",
        title: "エラー",
        description: message,
      });
    } else {
      // サーバーサイドの場合はコンソールにエラーを出力
      console.error("Unexpected Error:", error);
    }
  }
}

// エラーメッセージを取得する関数
function getErrorMessage(status: number): string {
  switch (status) {
    case 400:
      return "リクエストに問題があります。入力内容を確認してください。";
    case 401:
      return "認証に失敗しました。再度ログインしてください。";
    case 403:
      return "アクセス権限がありません。";
    case 404:
      return "リソースが見つかりません。";
    case 413:
      return "アップロードされたファイルが大きすぎます。より小さいファイルを選択してください。";
    case 422:
      return "入力内容に誤りがあります。確認して再度お試しください。";
    case 500:
      return "サーバーエラーが発生しました。しばらく経ってから再度お試しください。";
    case 502:
      return "アクセスが集中しています。しばらく経ってから再度お試しください。";
    default:
      return "予期せぬエラーが発生しました。";
  }
}
